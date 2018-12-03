use super::*;
use crate::test;

#[test]
fn execute_shell() {
    execute(
        &vec!["exit 0".to_string()],
        Some(test::get_os_runner()),
        test::get_os_extension(),
    );
}

#[test]
fn execute_shell_hello() {
    execute(
        &vec!["echo hello".to_string()],
        Some(test::get_os_runner()),
        test::get_os_extension(),
    );
}

#[test]
fn execute_shell_error() {
    execute(
        &vec!["exit 1".to_string()],
        Some(test::get_os_runner()),
        test::get_os_extension(),
    );
}

#[test]
fn extract_shebang_line_from_script() {
    let script_test = vec!["#!/usr/bin/env python".to_string(), "test".to_string()];
    let shebang = extract_runner_from_script(script_test).unwrap();
    assert_eq!("/usr/bin/env python", shebang);
}

#[test]
fn extract_runner_from_shebang_line() {
    let shebang = "#!/usr/bin/env python".to_string();
    let runner = extract_runner_from_shebang(shebang);
    assert_eq!("/usr/bin/env python", runner);
}


#[test]
fn execute_shell_with_shebang_line() {
    let script_test = vec!["#!/usr/bin/env python".to_string(), "print('hello from python')".to_string()];
    let extension = "py".to_string();
    let runner = extract_runner_from_script(script_test.clone()).unwrap();
    execute(
        &script_test,
        Some(runner),
        extension,
    );
}




