use super::*;

#[test]
fn create_text_file_and_delete() {
    let file = create_text_file("test\nend", ".testfile");
    assert!(file.ends_with(".testfile"));

    let mut file_obj = File::open(&file).unwrap();
    let mut text = String::new();
    file_obj.read_to_string(&mut text).unwrap();

    delete_file(&file);

    assert_eq!("test\nend".to_string(), text);
}
