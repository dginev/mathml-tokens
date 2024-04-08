use std::env;

use mathml_tokens::from_file;

fn main() {
  let mut args = env::args();
  let _self_path = args.next();
  let path = args.next().expect("Please provide an XML/HTML filepath on input");
  let tokens = from_file(&path, true)
    .expect("quick-xml processing shouldn't cause errors");
  println!("{}",tokens);
}