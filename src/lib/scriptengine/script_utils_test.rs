use super::*;
use crate::io;

#[test]
fn create_script_file_text() {
    let file =
        create_script_file(&vec!["test".to_string(), "end".to_string()], ".testfile").unwrap();
    assert!(file.ends_with(".testfile"));

    let text = fsio::file::read_text_file(&file).unwrap();

    io::delete_file(&file);

    assert_eq!("test\nend".to_string(), text);
}

#[test]
fn create_persisted_script_file_text() {
    let mut file =
        create_persisted_script_file(&vec!["test".to_string(), "end".to_string()], ".testfile")
            .unwrap();
    assert!(file.ends_with(".testfile"));
    let mut text = fsio::file::read_text_file(&file).unwrap();
    io::delete_file(&file);
    assert_eq!("test\nend".to_string(), text);

    let file1 =
        create_persisted_script_file(&vec!["test".to_string(), "end".to_string()], ".testfile")
            .unwrap();
    assert_eq!(file, file1);
    text = fsio::file::read_text_file(&file).unwrap();
    assert_eq!("test\nend".to_string(), text);
    io::delete_file(&file);

    file = create_persisted_script_file(&vec!["test2".to_string(), "end".to_string()], ".testfile")
        .unwrap();
    assert_ne!(file, file1);
    text = fsio::file::read_text_file(&file).unwrap();
    assert_eq!("test2\nend".to_string(), text);
    io::delete_file(&file);
}
