use crate::descriptor::env::merge_env;
use crate::types::EnvValue;
use indexmap::IndexMap;

#[test]
fn merge_env_both_empty() {
    let mut map1 = IndexMap::<String, EnvValue>::new();
    let mut map2 = IndexMap::<String, EnvValue>::new();

    let output = merge_env(&mut map1, &mut map2).expect("should have no cycle");
    assert_eq!(output.len(), 0);
}

#[test]
fn merge_env_reorder() {
    let mut map1 = IndexMap::new();
    map1.insert("env2".to_owned(), EnvValue::Value("${env1}".to_owned()));

    let mut map2 = IndexMap::new();
    map2.insert("env1".to_owned(), EnvValue::Value("x".to_owned()));

    let output = merge_env(&map1, &map2).expect("should have no cycle");
    assert_eq!(output.len(), 2);
    assert_eq!(output.get_index(0).unwrap().0.as_str(), "env1");
    assert_eq!(output.get_index(1).unwrap().0.as_str(), "env2");
}

#[test]
fn merge_env_reorder_internal() {
    let mut map1 = IndexMap::new();
    map1.insert("env3".to_owned(), EnvValue::Value("${env2}".to_owned()));
    map1.insert("env2".to_owned(), EnvValue::Value("${env1}".to_owned()));

    let mut map2 = IndexMap::new();
    map2.insert("env1".to_owned(), EnvValue::Value("x".to_owned()));

    let output = merge_env(&map1, &map2).expect("should have no cycle");
    assert_eq!(output.len(), 3);
    assert_eq!(output.get_index(0).unwrap().0.as_str(), "env1");
    assert_eq!(output.get_index(1).unwrap().0.as_str(), "env2");
    assert_eq!(output.get_index(2).unwrap().0.as_str(), "env3");
}

#[test]
fn merge_env_cycle() {
    let mut map1 = IndexMap::new();
    map1.insert("env2".to_owned(), EnvValue::Value("${env1}".to_owned()));

    let mut map2 = IndexMap::new();
    map2.insert("env1".to_owned(), EnvValue::Value("${env2}".to_owned()));

    let output = merge_env(&map1, &map2).expect_err("should have cycle");
    assert!(output.ends_with("env2 -> env1 -> env2."));
}

#[test]
fn merge_env_first_empty() {
    let mut map1 = IndexMap::<String, EnvValue>::new();
    let mut map2 = IndexMap::<String, EnvValue>::new();

    map2.insert("test".to_string(), EnvValue::Value("value".to_string()));

    let output = merge_env(&mut map1, &mut map2).expect("should have no cycle");
    assert_eq!(output.len(), 1);
    let value = output.get("test").unwrap();
    match value {
        &EnvValue::Value(ref value_string) => assert_eq!(value_string, &"value".to_string()),
        _ => panic!("wrong value type"),
    };
}

#[test]
fn merge_env_second_empty() {
    let mut map1 = IndexMap::<String, EnvValue>::new();
    let mut map2 = IndexMap::<String, EnvValue>::new();

    map1.insert("test".to_string(), EnvValue::Value("value".to_string()));

    let output = merge_env(&mut map1, &mut map2).expect("should have no cycle");
    assert_eq!(output.len(), 1);
    let value = output.get("test").unwrap();
    match value {
        &EnvValue::Value(ref value_string) => assert_eq!(value_string, &"value".to_string()),
        _ => panic!("wrong value type"),
    };
}

#[test]
fn merge_env_both_with_values() {
    let mut map1 = IndexMap::<String, EnvValue>::new();
    let mut map2 = IndexMap::<String, EnvValue>::new();

    map1.insert("test1".to_string(), EnvValue::Value("value1".to_string()));
    map1.insert("test21".to_string(), EnvValue::Value("value11".to_string()));
    map2.insert("test21".to_string(), EnvValue::Value("value21".to_string()));
    map2.insert("test22".to_string(), EnvValue::Value("value22".to_string()));

    let output = merge_env(&mut map1, &mut map2).expect("should have no cycle");
    assert_eq!(output.len(), 3);
    let mut value = output.get("test1").unwrap();
    match value {
        &EnvValue::Value(ref value_string) => assert_eq!(value_string, &"value1".to_string()),
        _ => panic!("wrong value type"),
    };
    value = output.get("test21").unwrap();
    match value {
        &EnvValue::Value(ref value_string) => assert_eq!(value_string, &"value21".to_string()),
        _ => panic!("wrong value type"),
    };
    value = output.get("test22").unwrap();
    match value {
        &EnvValue::Value(ref value_string) => assert_eq!(value_string, &"value22".to_string()),
        _ => panic!("wrong value type"),
    };
}

#[test]
fn merge_env_both_with_sub_envs() {
    let mut map1 = IndexMap::<String, EnvValue>::new();
    let mut map2 = IndexMap::<String, EnvValue>::new();

    map1.insert("test1".to_string(), EnvValue::Value("value1".to_string()));
    map1.insert("test21".to_string(), EnvValue::Value("value11".to_string()));
    map2.insert("test21".to_string(), EnvValue::Value("value21".to_string()));
    map2.insert("test22".to_string(), EnvValue::Value("value22".to_string()));

    let mut base_profile = IndexMap::<String, EnvValue>::new();
    let mut extended_profile = IndexMap::<String, EnvValue>::new();
    let mut extended_profile2 = IndexMap::<String, EnvValue>::new();

    base_profile.insert("base1".to_string(), EnvValue::Value("base1".to_string()));
    base_profile.insert("base2".to_string(), EnvValue::Value("base2".to_string()));
    extended_profile.insert(
        "base1".to_string(),
        EnvValue::Value("extended1".to_string()),
    );
    extended_profile.insert(
        "extended2".to_string(),
        EnvValue::Value("extended2".to_string()),
    );

    extended_profile2.insert("test".to_string(), EnvValue::Value("test1".to_string()));

    map1.insert("myprofile".to_string(), EnvValue::Profile(base_profile));
    map2.insert("myprofile".to_string(), EnvValue::Profile(extended_profile));
    map2.insert(
        "myprofile2".to_string(),
        EnvValue::Profile(extended_profile2),
    );

    let output = merge_env(&mut map1, &mut map2).expect("should have no cycle");
    assert_eq!(output.len(), 5);
    let mut value = output.get("test1").unwrap();
    match value {
        &EnvValue::Value(ref value_string) => assert_eq!(value_string, &"value1".to_string()),
        _ => panic!("wrong value type"),
    };
    value = output.get("test21").unwrap();
    match value {
        &EnvValue::Value(ref value_string) => assert_eq!(value_string, &"value21".to_string()),
        _ => panic!("wrong value type"),
    };
    value = output.get("test22").unwrap();
    match value {
        &EnvValue::Value(ref value_string) => assert_eq!(value_string, &"value22".to_string()),
        _ => panic!("wrong value type"),
    };
    let mut sub_env_type = output.get("myprofile").unwrap();
    match sub_env_type {
        EnvValue::Profile(sub_env) => {
            assert_eq!(sub_env.len(), 3);
            value = sub_env.get("base1").unwrap();
            match value {
                &EnvValue::Value(ref value_string) => {
                    assert_eq!(value_string, &"extended1".to_string())
                }
                _ => panic!("wrong value type"),
            };
            value = sub_env.get("base2").unwrap();
            match value {
                &EnvValue::Value(ref value_string) => {
                    assert_eq!(value_string, &"base2".to_string())
                }
                _ => panic!("wrong value type"),
            };
            value = sub_env.get("extended2").unwrap();
            match value {
                &EnvValue::Value(ref value_string) => {
                    assert_eq!(value_string, &"extended2".to_string())
                }
                _ => panic!("wrong value type"),
            };
        }
        _ => panic!("wrong value type"),
    }
    sub_env_type = output.get("myprofile2").unwrap();
    match sub_env_type {
        EnvValue::Profile(sub_env) => {
            assert_eq!(sub_env.len(), 1);
            value = sub_env.get("test").unwrap();
            match value {
                &EnvValue::Value(ref value_string) => {
                    assert_eq!(value_string, &"test1".to_string())
                }
                _ => panic!("wrong value type"),
            };
        }
        _ => panic!("wrong value type"),
    }
}

#[test]
fn merge_env_both_skip_current_task_env() {
    let mut map1 = IndexMap::<String, EnvValue>::new();
    let mut map2 = IndexMap::<String, EnvValue>::new();

    map1.insert("test1".to_string(), EnvValue::Value("value1".to_string()));
    map1.insert(
        "CARGO_MAKE_CURRENT_TASK_TEST".to_string(),
        EnvValue::Value("test1".to_string()),
    );
    map2.insert("test1".to_string(), EnvValue::Value("value2".to_string()));
    map2.insert(
        "CARGO_MAKE_CURRENT_TASK_TEST".to_string(),
        EnvValue::Value("test2".to_string()),
    );

    let output = merge_env(&mut map1, &mut map2).expect("should have no cycle");
    assert_eq!(output.len(), 2);
    let mut value = output.get("test1").unwrap();
    match value {
        &EnvValue::Value(ref value_string) => assert_eq!(value_string, &"value2".to_string()),
        _ => panic!("wrong value type"),
    };
    value = output.get("CARGO_MAKE_CURRENT_TASK_TEST").unwrap();
    match value {
        &EnvValue::Value(ref value_string) => assert_eq!(value_string, &"test1".to_string()),
        _ => panic!("wrong value type"),
    };
}
