use quick_xml::events::Event;
use quick_xml::reader::Reader;

pub enum VisitMode {
  Tokens,
  Skip(usize),
  TokensArg(usize),
}

// Kind of a strange deal... 
// MathML's extra verbosity comes from required element wrapper with single arguments, such as <mi>.
// That is the simple Tokens mode
// But for elements with two arguments, such as <msub>x 2</msub>, 
// argument scope and order carries important information. (TokensArg mode)

pub fn start_node_tokens(name:&[u8]) -> (&'static str,VisitMode) {
  use VisitMode::*;
  match name {
    b"math" | b"mrow" | b"mstyle" | b"mi" | b"mn" | b"mo" | b"ms" | b"mtext" | b"mpadded" => ("",Tokens),
    b"mspace" => (" ",Tokens),
    b"semantics" => ("",Tokens), // realized by virtue of skipping all non-presentation children
    b"mroot" => ("root",Tokens),
    b"msqrt" => ("sqrt",Tokens),
    b"merror" => ("error",Tokens),
    b"mtable" => ("table", Tokens),
    b"mtr" => ("tr", Tokens),
    b"mtd" => ("td", Tokens),
    b"mfrac" => ("frac",TokensArg(0)),
    b"mover" => ("over",TokensArg(0)),
    b"munder" => ("under",TokensArg(0)),
    b"munderover" => ("underover",TokensArg(0)),
    b"msub" => ("sub",TokensArg(0)),
    b"msubsup" => ("subsup",TokensArg(0)),
    b"msup" => ("sup",TokensArg(0)),
    b"mmultiscripts" => ("multiscripts",TokensArg(0)),
    b"mprescripts" => ("prescripts",TokensArg(0)),
    b"annotation" | b"annotation-xml" | b"maction" | b"mphantom" | b"apply" => ("",Skip(0)),
    other => {
      eprintln!("-- unexpected node start in MathML: {:?}; skipping.",other);
      ("", Skip(0))
    }
  }
}

pub fn end_node_tokens(name:&[u8]) -> (&'static str,VisitMode) {
  use VisitMode::*;
  match name {
    b"math" | b"mrow" | b"mstyle" | b"mi" | b"mn" | b"mo" | b"ms"| b"mtext" | b"mpadded" | b"mspace" => ("",Tokens),
    b"semantics" => ("",Tokens), // realized by virtue of skipping all non-presentation children
    b"mroot" => ("root",Tokens),
    b"msqrt" => ("sqrt",Tokens),
    b"merror" => ("error",Tokens),
    b"mtable" => ("table", Tokens),
    b"mtr" => ("tr", Tokens),
    b"mtd" => ("td", Tokens),
    other => {
      eprintln!("end {:?} should not have been in Tokens mode",other);
      ("",Tokens)
    }
  }
}

pub fn start_node_verbose(name:&[u8], lvl:usize) -> (&'static str,VisitMode) {
  use VisitMode::*;
  match name {
    b"mrow" | b"mstyle" | b"mi" | b"mn" | b"mo" | b"ms" | b"mtext" | b"mpadded" => ("arg",TokensArg(lvl)),
    b"mspace" => (" ",TokensArg(lvl)),
    b"mroot" => ("root",TokensArg(lvl)),
    b"msqrt" => ("sqrt",TokensArg(lvl)),
    b"merror" => ("error",TokensArg(lvl)),
    b"mtable" => ("table", TokensArg(lvl)),
    b"mtr" => ("tr", TokensArg(lvl)),
    b"mtd" => ("td", TokensArg(lvl)),
    b"mfrac" => ("frac",TokensArg(lvl+1)),
    b"mover" => ("over",TokensArg(lvl+1)),
    b"munder" => ("under",TokensArg(lvl+1)),
    b"munderover" => ("underover",TokensArg(lvl+1)),
    b"msub" => ("sub",TokensArg(lvl+1)),
    b"msubsup" => ("subsup",TokensArg(lvl+1)),
    b"msup" => ("sup",TokensArg(lvl+1)),
    b"mmultiscripts" => ("multiscripts",TokensArg(lvl+1)),
    b"mprescripts" => ("prescripts",TokensArg(lvl+1)),
    other => {
      eprintln!("Arg mode shouldn't have aux elements, found: {:?} and unwrapping", other);
      ("", TokensArg(lvl))
    }
  }
}

pub fn end_node_verbose(name:&[u8],lvl:usize) -> (&'static str,VisitMode) {
  use VisitMode::*;
  match name {
    b"mrow" | b"mstyle" | b"mi" | b"mn" | b"mo" | b"ms" | b"mtext" | b"mpadded" => ("arg",TokensArg(lvl)),
    b"mroot" => ("root",TokensArg(lvl)),
    b"msqrt" => ("sqrt",TokensArg(lvl)),
    b"merror" => ("error",TokensArg(lvl)),
    b"mspace" => ("",TokensArg(lvl)),
    b"mtable" => ("table", TokensArg(lvl)),
    b"mtr" => ("tr", TokensArg(lvl)),
    b"mtd" => ("td", TokensArg(lvl)),
    b"mfrac" => ("frac",if lvl==0 {Tokens} else {TokensArg(lvl-1)}),
    b"mover" => ("over",if lvl==0 {Tokens} else {TokensArg(lvl-1)}),
    b"munder" => ("under",if lvl==0 {Tokens} else {TokensArg(lvl-1)}),
    b"munderover" => ("underover",if lvl==0 {Tokens} else {TokensArg(lvl-1)}),
    b"mmultiscripts" => ("multiscripts",if lvl==0 {Tokens} else {TokensArg(lvl-1)}),
    b"mprescripts" => ("prescripts",if lvl==0 {Tokens} else {TokensArg(lvl-1)}),
    b"msub" => ("sub",if lvl==0 {Tokens} else {TokensArg(lvl-1)}),
    b"msubsup" => ("subsup",if lvl==0 {Tokens} else {TokensArg(lvl-1)}),
    b"msup" => ("sup",if lvl==0 {Tokens} else {TokensArg(lvl-1)}),
    other => {
      eprintln!("Arg mode shouldn't have aux elements, found: {:?} and unwrapping", other);
      ("", TokensArg(lvl))
    }
  }
}

pub fn end_node_skip(name:&[u8], lvl:usize) -> (&'static str, VisitMode) {
  use VisitMode::*;
  match name {
    b"annotation" | b"annotation-xml" | b"maction" | b"mphantom" => ("", if lvl==0 {Tokens} else {Skip(lvl-1)}),
    _ => ("", Skip(lvl)),
  }
}

const END_PREFIX: &str = "end_";
const TOKEN_START_CHAR :char = '[';
const TOKEN_END_CHAR :char = ']';

pub fn from_str(xml:&str) -> String {
  use VisitMode::*;
  let mut reader = Reader::from_str(xml);
  reader.trim_text(true);
  let mut tokens = String::new();
  let mut mode = Tokens;
  // 1. some elements cause skips until their end
  // 2. some elmeents cause "verbose mode" where their arguments need an [arg] [end_arg] wrapper.
  // DFS traverse, process events and create the tokens
  loop {
    let event = reader.read_event().unwrap();
    let is_end = matches!(event,Event::End(_));
    let (piece, next_mode) = match (mode, event) {
      (Tokens, Event::Start(e)) => start_node_tokens(e.name().as_ref()),
      (Tokens, Event::End(e)) => end_node_tokens(e.name().as_ref()),
      (Skip(lvl), Event::End(e)) => end_node_skip(e.name().as_ref(), lvl),
      (Skip(lvl), _) => ("",Skip(lvl)),
      (Tokens, Event::Text(e)) => {
        // handle text right away to avoid borrowing issues
        if !tokens.is_empty() { tokens.push(' ');}
        tokens.push_str(e.unescape().unwrap().as_ref());
        ("",Tokens) },
      (TokensArg(lvl), Event::Text(e)) => {
        // handle text right away to avoid borrowing issues
        if !tokens.is_empty() { tokens.push(' ');}
        tokens.push_str(e.unescape().unwrap().as_ref());
        ("", TokensArg(lvl)) },
      (TokensArg(lvl), Event::Start(e)) => start_node_verbose(e.name().as_ref(), lvl),
      (TokensArg(lvl), Event::End(e)) => end_node_verbose(e.name().as_ref(), lvl),
      (keep_mode, Event::PI(_)) => ("",keep_mode),
      (keep_mode, Event::CData(_)) => ("",keep_mode),
      (keep_mode, Event::DocType(_)) => ("",keep_mode),
      (keep_mode, Event::Comment(_)) => ("",keep_mode),
      (keep_mode, Event::Empty(_)) => ("",keep_mode),
      (keep_mode, Event::Decl(_)) => ("",keep_mode),
      (_, Event::Eof) => break,
    };
    if !piece.is_empty() {
      if !tokens.is_empty() {
        tokens.push(' ');
      }
      tokens.push(TOKEN_START_CHAR);
      if is_end {
        tokens.push_str(END_PREFIX);
      }
      tokens.push_str(piece);
      tokens.push(TOKEN_END_CHAR);
    }
    mode = next_mode;
  }
  tokens
}
