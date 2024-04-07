use std::error::Error;
use std::io::BufRead;

use quick_xml::events::Event;
use quick_xml::reader::Reader;

#[derive(Debug,Copy,Clone,PartialEq)]
pub enum VisitMode {
  Tokens,
  Skip,
  Args,
  Unwrap,
}

// Kind of a strange deal...
// MathML's extra verbosity comes from required element wrapper with single arguments, such as <mi>.
// That is the simple Tokens mode
// But for elements with two arguments, such as <msub>x 2</msub>,
// argument scope and order carries important information. (Args mode)

pub fn node_to_token(name:&[u8]) -> (&'static str,VisitMode) {
  use VisitMode::*;
  match name {
    b"math" => ("math",Tokens),
    b"mrow" | b"mstyle" | b"mi" | b"mn" | b"mo" | b"ms" | b"mtext" | b"mpadded" => ("",Tokens),
    b"semantics" => ("",Unwrap), // "semantics" realized by virtue of skipping all non-presentation children
    b"mspace" => (" ",Tokens),
    b"mroot" => ("root",Tokens),
    b"msqrt" => ("sqrt",Tokens),
    b"merror" => ("error",Tokens),
    b"mtable" => ("table", Tokens),
    b"mtr" => ("tr", Tokens),
    b"mtd" => ("td", Tokens),
    b"mfrac" => ("frac",Args),
    b"mover" => ("over",Args),
    b"munder" => ("under",Args),
    b"munderover" => ("underover",Args),
    b"msub" => ("sub",Args),
    b"msubsup" => ("subsup",Args),
    b"msup" => ("sup",Args),
    b"mmultiscripts" => ("multiscripts",Args),
    b"mprescripts" => ("prescripts",Args),
    // Content MathML
    b"annotation" | b"annotation-xml" | b"maction" | b"mphantom" | b"apply" |
    b"vector" | b"matrix" | b"interval" | b"gt" | b"lt" | b"in" | b"set" | b"leq" | b"geq" |
    b"plus" | b"log" | b"sum" | b"floor" | b"matrixrow" | b"cerror" | b"approx" | b"and" |
    b"share" | b"max" | b"min" | b"list" | b"limit" | b"infinity" | b"neq" | b"ceiling" |
    b"union" | b"none" | b"abs" |
    b"eq" | b"divide" | b"root" | b"csymbol" | b"minus" | b"ci" | b"cn" | b"times" => ("",Skip),
    // HTML Realm
    b"span" | b"em" => ("",Unwrap), // some regular HTML elements may show up inside <mtext>
    b"html" | b"body" | b"div" | b"li" | b"ul" | b"ol" | b"p" | b"a" |
    b"h1" | b"h2" | b"h3" | b"h4" | b"h5" | b"h6" | b"h7" | b"table" | b"tr" | b"td" |
    b"article" | b"section" | b"tbody" | b"sup" | b"sub" | b"figure" | b"figcaption" => ("",Unwrap),
    b"svg" | b"img" | b"mglyph" => ("image", Skip),
    b"head" | b"meta" | b"title" | b"cite" => ("",Skip),
    other => {
      eprintln!("-- unexpected node start in MathML: {}; skipping.", String::from_utf8_lossy(other));
      ("", Skip)
    }
  }

}

const END_PREFIX: &str = "end_";
const TOKEN_START_CHAR :char = '[';
const TOKEN_END_CHAR :char = ']';

pub fn from_file(path:&str, math_token: bool) -> Result<String, Box<dyn Error>> {
  let mut reader = Reader::from_file(path)?;
  reader.trim_text(true);
  from_reader(reader,math_token)
}

pub fn from_str(xml:&str, math_token: bool) -> Result<String, Box<dyn Error>> {
  let mut reader = Reader::from_str(xml);
  reader.trim_text(true);
  from_reader(reader,math_token)
}
pub fn from_reader<T:BufRead>(mut reader: Reader<T>,math_token: bool) -> Result<String, Box<dyn Error>> {

  let mut reader_buf = Vec::new();
  use VisitMode::*;
  let mut tokens = String::new();
  let mut modes = vec![Unwrap];
  let mut outer_stack = Vec::new();
  let mut prepared_args: Vec<String>  = Vec::new();


  // 1. some elements cause skips until their end
  // 2. some elmeents cause "verbose mode" where their arguments need an [arg] [end_arg] wrapper.
  // DFS traverse, process events and create the tokens
  loop {
    let event = reader.read_event_into(&mut reader_buf)?;
    let mut is_start = false;
    let mut is_end = false;
    let current_mode = *modes.last().unwrap();
    let (piece, mut node_mode) = match event {
      Event::Start(e) => {is_start=true; node_to_token(e.name().as_ref())},
      Event::End(e) => {is_end=true; node_to_token(e.name().as_ref()) },
      Event::Text(e) => {
        if current_mode!=Skip && current_mode != Unwrap {
          // trim text contents
          let text_trim = e.unescape().unwrap().as_ref().trim().replace('\u{2062}', "");
          if !text_trim.is_empty() {
            if !tokens.is_empty() { tokens.push(' ');}
            tokens.push_str(&text_trim);
          }
        }
        continue; },
      Event::PI(_) | Event::CData(_) | Event::DocType(_) | Event::Comment(_) |
      Event::Empty(_) | Event::Decl(_) => ("",current_mode),
      Event::Eof => break,
    };
    // unwrapping in math preserves tokens, but not *outside* of math
    if node_mode == Unwrap && (current_mode == Tokens || current_mode == Args) {
      node_mode = current_mode;
    }
    // Handle STARTs of tags
    if !piece.is_empty() && is_start && current_mode != Skip {
      if !tokens.is_empty() && piece != "math" {
        tokens.push(' ');
      }
      if math_token || piece != "math" {
        tokens.push(TOKEN_START_CHAR);
        tokens.push_str(piece);
        tokens.push(TOKEN_END_CHAR);
      }
    }
    // Enact a mode switch when opening an element.
    // start a new frame when the mode is ARG.
    if is_start {
      modes.push(node_mode);
      if node_mode == Args {
        outer_stack.push((prepared_args,tokens));
        prepared_args = Vec::new();
        tokens = String::new();
      }
    } else if is_end {
      // 1. Finalize the current mode when ending an element.
      if current_mode == node_mode {
        modes.pop();
      } else {
        eprintln!("-- Mode mismatch! Tried to end {node_mode:?} but current is {current_mode:?}.");
      }
      // 2. Finalize inner: if we just ended an ARG element, prepare its arguments and unwind the frame.
      if node_mode == Args {
        if !tokens.is_empty() {
          // add to the latest arg, or push new
          if let Some(last_arg) = prepared_args.pop() {
            if last_arg.is_empty() {
              prepared_args.push(tokens);
            } else {
              prepared_args.push(last_arg+" "+&tokens)
            }
          } else {
            prepared_args.push(tokens);
          }
        }
        let mut inner_buff = String::new();
        for prepared_arg in prepared_args.iter() {
          // Simplified treatment: unwrap 1-token argument groups, such as [arg] 1 [end_arg] --> 1
          // helps to reduce the token count.
          let is_complex = prepared_arg.contains(' ');
          if !inner_buff.is_empty() { inner_buff.push(' '); }
          if is_complex {
            inner_buff.push(TOKEN_START_CHAR);
            inner_buff.push_str("arg");
            inner_buff.push(TOKEN_END_CHAR);
            inner_buff.push(' ');
          }
          inner_buff.push_str(prepared_arg);
          if is_complex {
            inner_buff.push(' ');
            inner_buff.push(TOKEN_START_CHAR);
            inner_buff.push_str(END_PREFIX);
            inner_buff.push_str("arg");
            inner_buff.push(TOKEN_END_CHAR);
          }
        }
        let (outer_prepared_args, outer_tokens) = outer_stack.pop().unwrap_or_default();
        prepared_args = outer_prepared_args;
        tokens = if outer_tokens.is_empty() {
          inner_buff
        } else {
          outer_tokens + " " + &inner_buff
        };
      }
      // 3. Finalize outer: if we were acting inside an ARG parent, save the current buffer as a prepared argument.
      if modes.last() == Some(&Args) {
        prepared_args.push(tokens);
        tokens = String::new();
      }
      // Handle ENDs of tags
      if !piece.is_empty() && current_mode != Skip && (math_token || piece != "math") {
        if !tokens.is_empty() {
          tokens.push(' ');
        }
        tokens.push(TOKEN_START_CHAR);
        tokens.push_str(END_PREFIX);
        tokens.push_str(piece);
        tokens.push(TOKEN_END_CHAR);     
        if math_token && piece == "math" {
          tokens.push('\n');
          tokens.push('\n');
        }
      }
    }
 }
 Ok(tokens)
}
