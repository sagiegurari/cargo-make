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

#[test]
fn extract_runner_from_bash_script() {
    let script_test = vec!["#!/usr/bin/env bash".to_string(), "test".to_string()];
    let shebang = extract_runner_from_script(script_test).unwrap();
    assert_eq!("/usr/bin/env bash", shebang);
}

#[test]
fn extract_runner_from_shebang_line() {
    let shebang = "#!/usr/bin/env bash".to_string();
    let runner = extract_runner_from_shebang(shebang);
    println!("{}", runner);
    assert_eq!("/usr/bin/env bash", runner);
}
