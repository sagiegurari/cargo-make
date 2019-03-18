use super::*;

use std::env;

#[test]
#[should_panic]
fn run_function_empty() {
    run_function("", &vec![]);
}

#[test]
#[should_panic]
fn run_function_not_exists() {
    run_function("bad", &vec![]);
}

#[test]
fn run_function_split() {
    env::set_var("TEST_SPLIT_FUNC_MOD", "1,2,3,4");

    let output = run_function(
        "split",
        &vec!["TEST_SPLIT_FUNC_MOD".to_string(), ",".to_string()],
    );

    assert_eq!(output, vec!["1", "2", "3", "4"]);
}

#[test]
fn run_function_remove_empty() {
    env::set_var("TEST_REMOVE_EMPTY_FUNC_MOD", "");

    let output = run_function(
        "remove-empty",
        &vec!["TEST_REMOVE_EMPTY_FUNC_MOD".to_string()],
    );

    assert_eq!(output.len(), 0);
}

#[test]
fn get_function_name_valid() {
    let output = get_function_name("test(123)");

    assert_eq!(output.unwrap(), "test");
}

#[test]
fn get_function_name_invalid() {
    let output = get_function_name("test[123]");

    assert!(output.is_none());
}

#[test]
fn get_function_argument_empty() {
    let output = get_function_argument("");

    assert_eq!(output, "");
}

#[test]
fn get_function_argument_single_char() {
    let output = get_function_argument(" ");

    assert_eq!(output, " ");
}

#[test]
fn get_function_argument_spaces() {
    let output = get_function_argument("     ");

    assert_eq!(output, "");
}

#[test]
fn get_function_argument_mixed() {
    let output = get_function_argument(" |");

    assert_eq!(output, "|");
}

#[test]
fn get_function_arguments_missing_start() {
    let output = get_function_arguments("1,2,3,4)");

    assert!(output.is_none());
}

#[test]
fn get_function_arguments_missing_end() {
    let output = get_function_arguments("(1,2,3,4");

    assert!(output.is_none());
}

#[test]
fn get_function_arguments_empty() {
    let output = get_function_arguments("()");

    let expected: Vec<String> = vec![];
    assert_eq!(output.unwrap(), expected);
}

#[test]
fn get_function_arguments_single() {
    let output = get_function_arguments("(1)");

    assert_eq!(output.unwrap(), vec!["1"]);
}

#[test]
fn get_function_arguments_multiple() {
    let output = get_function_arguments("(1,2,3)");

    assert_eq!(output.unwrap(), vec!["1", "2", "3"]);
}

#[test]
fn get_function_arguments_multiple_with_spaces() {
    let output = get_function_arguments("(1  ,  2,   3   )");

    assert_eq!(output.unwrap(), vec!["1", "2", "3"]);
}

#[test]
fn evaluate_and_run_valid() {
    env::set_var("TEST_RUN_FUNC_VALUE", "1 2 3 4");

    let output = evaluate_and_run("@@split(TEST_RUN_FUNC_VALUE, )");

    assert_eq!(output, vec!["1", "2", "3", "4"]);
}

#[test]
#[should_panic]
fn evaluate_and_run_unknown_function() {
    evaluate_and_run("@@bad()");
}

#[test]
fn evaluate_and_run_no_function() {
    let output = evaluate_and_run("value");

    assert_eq!(output, vec!["value"]);
}

#[test]
fn modify_arguments_with_functions() {
    env::set_var("TEST_MOD_ARGS_FUNC_VALUE", "1|2|3|4");

    let mut task = Task::new();
    task.args = Some(vec![
        "start".to_string(),
        "@@split(TEST_MOD_ARGS_FUNC_VALUE, |)".to_string(),
        "end".to_string(),
    ]);

    modify_arguments(&mut task);

    assert_eq!(task.args.unwrap(), vec!["start", "1", "2", "3", "4", "end"]);
}

#[test]
fn run_with_functions() {
    env::set_var("TEST_STEP_FUNC_VALUE", "1 2 3 4");

    let mut task = Task::new();
    task.args = Some(vec![
        "start".to_string(),
        "@@split(TEST_STEP_FUNC_VALUE, )".to_string(),
        "end".to_string(),
    ]);
    let mut step = Step {
        name: "test".to_string(),
        config: task,
    };

    step = run(&step);

    assert_eq!(
        step.config.args.unwrap(),
        vec!["start", "1", "2", "3", "4", "end"]
    );
}
