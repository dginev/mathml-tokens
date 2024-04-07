use std::fs::File;
use std::io::Read;

use mathml_tokens::from_file;

#[test]
fn arxiv_math0703429_to_tokens() {
  let tokens = from_file("tests/samples/math0703429.html", true).expect("quick-xml processing shouldn't cause errors");
  let mut target_tokens_file = File::open("tests/samples/math0703429.txt").expect("target TXT expected to be present on disk.");
  let mut target_tokens = String::new();
  target_tokens_file.read_to_string(&mut target_tokens).expect("target file read shouldn't cause errors");
  let tokens_vec = tokens.split("\n\n");
  let target_vec = target_tokens.split("\n\n");
  for (token_expr, target_expr) in tokens_vec.zip(target_vec) {
    assert_eq!(token_expr, target_expr);
  }
}