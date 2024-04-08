use std::error::Error;
use std::io::BufRead;

use wasm_bindgen::prelude::*;

use quick_xml::events::Event;
use quick_xml::reader::Reader;
use web_sys::Element;

#[derive(Debug,Copy,Clone,PartialEq)]
pub enum VisitMode {
  Tokens,
  Skip,
  Args,
  Unwrap,
}

#[derive(Debug,Copy,Clone,PartialEq)]
pub enum VisitEvent {
  Start,
  End,
  Text,
  Other
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
    b"union" | b"none" | b"abs" | b"notin" | b"subset" | b"intersect" | b"emptyset" | b"compose" |
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

fn text_to_tokens(runtime: &mut Runtime, text:&str) {
  use VisitMode::*;
  let Runtime {current_mode, ref mut tokens, ..} = runtime;
  if !matches!(current_mode, Skip | Unwrap) {
    // trim text contents
    let text_trim = text.trim().replace('\u{2062}', "");
    if !text_trim.is_empty() {
      if !tokens.is_empty() { tokens.push(' ');}
      tokens.push_str(&text_trim);
    }
  }
}

struct Runtime {
  current_mode: VisitMode,
  event:VisitEvent,
  math_token: bool,
  modes: Vec<VisitMode>,
  outer_stack: Vec<(Vec<String>,String)>,
  prepared_args: Vec<String>,
  tokens: String,
}

fn build_tokens(runtime:&mut Runtime, piece: &str, mut node_mode: VisitMode) {
  use VisitMode::*;
  use VisitEvent::*;
  let Runtime {current_mode, event, 
    math_token, modes, prepared_args, outer_stack, ref mut tokens} = runtime;
  // unwrapping in math preserves tokens, but not *outside* of math
  if matches!(node_mode, Unwrap) && matches!(current_mode, Tokens | Args) {
    node_mode = *current_mode;
  } else if matches!(current_mode, Skip) {
    // Skip applies to all descendants, until it is popped.  
    node_mode = Skip;
  }
  // Handle STARTs of tags
  if !piece.is_empty() && matches!(event,Start) && !matches!(current_mode, Skip) {
    if !tokens.is_empty() && piece != "math" {
      tokens.push(' ');
    }
    if *math_token || piece != "math" {
      tokens.push(TOKEN_START_CHAR);
      tokens.push_str(piece);
      tokens.push(TOKEN_END_CHAR);
    }
  }
  // Enact a mode switch when opening an element.
  // start a new frame when the mode is ARG.
  if matches!(event,Start) {
    modes.push(node_mode);
    if matches!(node_mode, Args) {
      let mut swap_tokens = String::new();
      std::mem::swap(tokens,&mut swap_tokens);
      let mut swap_args = Vec::new();
      std::mem::swap(prepared_args,&mut swap_args);  
      outer_stack.push((swap_args,swap_tokens));
    }
  } else if matches!(event, End) {
    // 1. Finalize the current mode when ending an element.
    if *current_mode == node_mode {
      modes.pop();
    } else {
      eprintln!("-- Mode mismatch! Tried to end {node_mode:?} but current is {current_mode:?}.");
    }
    // 2. Finalize inner: if we just ended an ARG element, prepare its arguments and unwind the frame.
    if matches!(node_mode, Args) {
      if !tokens.is_empty() {
        // add to the latest arg, or push new
        if let Some(last_arg) = prepared_args.pop() {
          if last_arg.is_empty() {
            let mut swap = String::new();
            std::mem::swap(tokens, &mut swap);
            prepared_args.push(swap);
          } else {
            prepared_args.push(last_arg+" "+tokens);
          }
        } else {
          let mut swap = String::new();
          std::mem::swap(tokens, &mut swap);
          prepared_args.push(swap);
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
      *prepared_args = outer_prepared_args;
      *tokens = if outer_tokens.is_empty() {
        inner_buff
      } else {
        outer_tokens + " " + &inner_buff
      };
    }
    // 3. Finalize outer: if we were acting inside an ARG parent, save the current buffer as a prepared argument.
    if modes.last() == Some(&Args) {
      let mut swap = String::new();
      std::mem::swap(tokens, &mut swap);
      prepared_args.push(swap);
    }
    // Handle ENDs of tags
    if !piece.is_empty() && !matches!(*current_mode, Skip) && (*math_token || piece != "math") {
      if !tokens.is_empty() {
        tokens.push(' ');
      }
      tokens.push(TOKEN_START_CHAR);
      tokens.push_str(END_PREFIX);
      tokens.push_str(piece);
      tokens.push(TOKEN_END_CHAR);     
      if *math_token && piece == "math" {
        tokens.push('\n');
        tokens.push('\n');
      }
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
  let mut runtime = Runtime {
    tokens: String::new(),
    modes: vec![Unwrap],
    outer_stack: Vec::new(),
    prepared_args: Vec::new(),
    math_token,
    current_mode:Unwrap,
    event: VisitEvent::Other
  };
  // 1. some elements cause skips until their end
  // 2. some elmeents cause "verbose mode" where their arguments need an [arg] [end_arg] wrapper.
  // DFS traverse, process events and create the tokens
  loop {
    runtime.event = VisitEvent::Other;
    runtime.current_mode = *(runtime.modes.last().unwrap());
    let event = reader.read_event_into(&mut reader_buf)?;
    let (piece, node_mode) = match event {
      Event::Start(e) => {
        runtime.event = VisitEvent::Start;
        node_to_token(e.name().as_ref()) },
      Event::End(e) => {
        runtime.event = VisitEvent::End;
        node_to_token(e.name().as_ref())
      },
      Event::Text(e) => {
        text_to_tokens(&mut runtime, e.unescape().unwrap().as_ref());
        continue; },
      Event::PI(_) | Event::CData(_) | Event::DocType(_) | Event::Comment(_) |
      Event::Empty(_) | Event::Decl(_) => ("",runtime.current_mode),
      Event::Eof => break,
    };
    build_tokens(&mut runtime, piece, node_mode);
 }
 Ok(runtime.tokens)
}

#[wasm_bindgen]
pub fn mathml_str_to_tokens(serialized: &str) -> String {
  from_str(serialized, false).expect("dirty fail for WASM")
}

#[wasm_bindgen]
pub fn html_str_to_tokens(serialized: &str) -> String {
  from_str(serialized, true).expect("dirty fail for WASM")
}

#[wasm_bindgen]
pub fn node_to_tokens(root: Element) -> String {
  root.node_name()
}