use super::*;

#[test]
fn create_text_file_read_and_delete() {
    let file = create_text_file("test\nend", ".testfile");
    assert!(file.ends_with(".testfile"));

    let mut file_obj = File::open(&file).unwrap();
    let mut text = String::new();
    file_obj.read_to_string(&mut text).unwrap();

    let mut file_path = PathBuf::new();
    file_path.push(&file);
    let read_text = read_text_file(&file_path);

    delete_file(&file);

    assert_eq!("test\nend".to_string(), text);
    assert_eq!(read_text, text);
}

#[test]
fn write_text_file_read_and_delete() {
    let file = "./target/_temp/file.txt";
    let written = write_text_file(&file, "test\nend");
    assert!(written);

    let mut file_obj = File::open(&file).unwrap();
    let mut text = String::new();
    file_obj.read_to_string(&mut text).unwrap();

    let mut file_path = PathBuf::new();
    file_path.push(&file);
    let read_text = read_text_file(&file_path);

    delete_file(&file);

    assert_eq!("test\nend".to_string(), text);
    assert_eq!(read_text, text);
}
