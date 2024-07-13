use super::*;

#[test]
fn create_text_file_read_and_delete() {
    let file = create_text_file("test\nend", ".testfile").unwrap();
    assert!(file.ends_with(".testfile"));

    let text = fsio::file::read_text_file(&file).unwrap();

    let mut file_path = PathBuf::new();
    file_path.push(&file);
    let read_text = read_text_file(&file_path).unwrap();

    delete_file(&file);

    assert_eq!("test\nend".to_string(), text);
    assert_eq!(read_text, text);
}

#[test]
fn write_text_file_read_and_delete() {
    let file = "./target/_temp/file.txt";
    let written = write_text_file(&file, "test\nend");
    assert!(written);

    let text = fsio::file::read_text_file(&file).unwrap();

    let mut file_path = PathBuf::new();
    file_path.push(&file);
    let read_text = read_text_file(&file_path).unwrap();

    delete_file(&file);

    assert_eq!("test\nend".to_string(), text);
    assert_eq!(read_text, text);
}

#[test]
fn get_path_list_not_exists() {
    let output = get_path_list("./target2", true, true, None);

    assert!(output.is_empty());
}

#[test]
fn get_path_list_files() {
    let output = get_path_list("./src/*_test.rs", true, true, None);

    let set: HashSet<String> = HashSet::from_iter(output.iter().cloned());
    assert_eq!(
        set,
        HashSet::from_iter(vec![
            "./src/main_test.rs".to_string(),
            "./src/makers_test.rs".to_string()
        ])
    );
}

#[test]
fn get_path_list_files_exclude_files() {
    let output = get_path_list("./src/*_test.rs", false, true, None);

    assert!(output.is_empty());
}

#[test]
fn get_path_list_dirs() {
    let output = get_path_list("./src/l*", true, true, None);

    let set: HashSet<String> = HashSet::from_iter(output.iter().cloned());
    assert_eq!(set, HashSet::from_iter(vec!["./src/lib".to_string(),]));
}

#[test]
fn get_path_list_dirs_exclude_dirs() {
    let output = get_path_list("./src/l*", true, false, None);

    assert!(output.is_empty());
}

#[test]
fn get_path_list_files_and_dirs() {
    let output = get_path_list("./src/*i*", true, true, None);

    let set: HashSet<String> = HashSet::from_iter(output.iter().cloned());
    assert_eq!(
        set,
        HashSet::from_iter(vec![
            "./src/lib".to_string(),
            "./src/main.rs".to_string(),
            "./src/main_test.rs".to_string(),
        ])
    );
}

#[test]
fn get_path_list_dirs_without_gitignore() {
    let output = get_path_list("./target", true, true, None);

    let set: HashSet<String> = HashSet::from_iter(output.iter().cloned());
    assert_eq!(set, HashSet::from_iter(vec!["./target".to_string(),]));
}

#[test]
fn get_path_list_dirs_with_gitignore() {
    let output = get_path_list("./target", true, true, Some("git".to_string()));

    assert!(output.is_empty());
}

#[test]
#[should_panic]
fn get_path_list_dirs_with_wrong_include_file_type() {
    get_path_list("./target", true, true, Some("bad".to_string()));
}
