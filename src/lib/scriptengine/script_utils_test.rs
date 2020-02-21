use super::*;
use crate::io;
use fsio;

#[test]
fn create_script_file_text() {
    let file = create_script_file(&vec!["test".to_string(), "end".to_string()], ".testfile");
    assert!(file.ends_with(".testfile"));

    let text = fsio::file::read_text_file(&file).unwrap();

    io::delete_file(&file);

    assert_eq!("test\nend".to_string(), text);
}
