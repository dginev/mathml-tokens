use std::fs::File;
use std::io::Read;

use mathml_tokens::from_file;

#[test]
fn arxiv_files_to_tokens() {
  let file_ids = ["math0703429","1104.2640","2304.02337"];
  for file_id in file_ids {
    let tokens = from_file(&format!("tests/samples/{file_id}.html"), true).expect("quick-xml processing shouldn't cause errors");
    let mut target_tokens_file = File::open(&format!("tests/samples/{file_id}.txt")).expect("target TXT expected to be present on disk.");
    let mut target_tokens = String::new();
    target_tokens_file.read_to_string(&mut target_tokens).expect("target file read shouldn't cause errors");
    let mut tokens_split = tokens.split("\n\n");
    let mut target_split = target_tokens.split("\n\n");
    let mut index=0;
    while let (Some(token_expr),Some(target_expr)) = (tokens_split.next(),target_split.next()) {
      index+=1;
      assert_eq!(token_expr, target_expr,"Tokenization of expression {index} in {file_id} differs.");
    }
    assert_eq!(tokens_split.next(),None,"Processed expression {} in {} was NOT in target set.",index+1,file_id);
    assert_eq!(target_split.next(),None,"Target expression {} in {} was NOT covered.",index+1,file_id);
  }
}