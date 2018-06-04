use super::*;

#[test]
fn create_script_file_and_delete() {
    let file = create_script_file(&vec!["test".to_string(), "end".to_string()], ".testfile");
    assert!(file.ends_with(".testfile"));

    let mut file_obj = File::open(&file).unwrap();
    let mut text = String::new();
    file_obj.read_to_string(&mut text).unwrap();

    assert_eq!("test\nend".to_string(), text);
}
