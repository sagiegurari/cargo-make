use super::*;

use crate::types::{ConfigSection, EnvValue, Task};
use expect_test::{expect, Expect};
use indexmap::IndexMap;
use std::path::PathBuf;

fn check(
    config: &Config,
    output_format: &str,
    output_file: &Option<String>,
    category: Option<String>,
    hide_uninteresting: bool,
    expect: Expect,
) {
    match output_file {
        Some(file) => {
            run(
                &config,
                output_format,
                output_file,
                category,
                hide_uninteresting,
            );

            let mut path = PathBuf::new();
            path.push(&file);

            let actual = io::read_text_file(&path);
            io::delete_file(&file);

            expect.assert_eq(&actual);
        }
        None => {
            let actual = create_list(&config, output_format, category, hide_uninteresting);
            expect.assert_eq(&actual);
        }
    }
}

#[test]
fn run_empty() {
    let config_section = ConfigSection::new();
    let env = IndexMap::<String, EnvValue>::new();
    let tasks = IndexMap::<String, Task>::new();
    let config = Config {
        config: config_section,
        env_files: vec![],
        env,
        env_scripts: vec![],
        tasks,
        plugins: None,
    };

    check(&config, "default", &None, None, false, expect![[""]]);
}

#[test]
fn run_all_public() {
    let config_section = ConfigSection::new();
    let env = IndexMap::<String, EnvValue>::new();

    let mut tasks = IndexMap::<String, Task>::new();
    let mut task1 = Task::new();
    task1.description = Some("1".to_string());
    tasks.insert("pre-1".to_string(), task1);
    let mut task2 = Task::new();
    task2.description = Some("2".to_string());
    tasks.insert("post-2".to_string(), task2);

    let config = Config {
        config: config_section,
        env_files: vec![],
        env,
        env_scripts: vec![],
        tasks,
        plugins: None,
    };

    check(
        &config,
        "default",
        &None,
        None,
        false,
        expect![[r#"
        No Category
        ----------
        post-2 - 2
        pre-1 - 1

    "#]],
    );
}

#[test]
fn run_all_public_hide_uninteresting() {
    let config_section = ConfigSection::new();
    let env = IndexMap::<String, EnvValue>::new();

    let mut tasks = IndexMap::<String, Task>::new();
    let mut task1 = Task::new();
    task1.description = Some("1".to_string());
    tasks.insert("pre-1".to_string(), task1);
    let mut task2 = Task::new();
    task2.description = Some("2".to_string());
    tasks.insert("post-2".to_string(), task2);
    let mut task3 = Task::new();
    task3.description = Some("3".to_string());
    tasks.insert("3".to_string(), task3);

    let config = Config {
        config: config_section,
        env_files: vec![],
        env,
        env_scripts: vec![],
        tasks,
        plugins: None,
    };

    check(
        &config,
        "default",
        &None,
        None,
        true,
        expect![[r#"
        No Category
        ----------
        3 - 3

    "#]],
    );
}

#[test]
fn run_aliases() {
    let config_section = ConfigSection::new();
    let env = IndexMap::<String, EnvValue>::new();

    let mut tasks = IndexMap::<String, Task>::new();
    let mut task1 = Task::new();
    task1.description = Some("1".to_string());
    tasks.insert("1".to_string(), task1);
    let mut task2 = Task::new();
    task2.description = Some("2".to_string());
    tasks.insert("2".to_string(), task2);
    let mut task3 = Task::new();
    // 4->3->1
    task3.description = Some("3".to_string());
    task3.alias = Some("1".to_string());
    tasks.insert("3".to_string(), task3);
    let mut task4 = Task::new();
    task4.description = Some("4".to_string());
    task4.alias = Some("3".to_string());
    tasks.insert("4".to_string(), task4);

    let config = Config {
        config: config_section,
        env_files: vec![],
        env,
        env_scripts: vec![],
        tasks,
        plugins: None,
    };

    check(
        &config,
        "default",
        &None,
        None,
        false,
        expect![[r#"
            No Category
            ----------
            1 - 1 [aliases: 3, 4]
            2 - 2

        "#]],
    );
}

#[test]
fn run_all_public_markdown() {
    let config_section = ConfigSection::new();
    let env = IndexMap::<String, EnvValue>::new();

    let mut tasks = IndexMap::<String, Task>::new();
    let mut task1 = Task::new();
    task1.description = Some("1".to_string());
    tasks.insert("1".to_string(), task1);
    let mut task2 = Task::new();
    task2.description = Some("2".to_string());
    tasks.insert("2".to_string(), task2);

    let config = Config {
        config: config_section,
        env_files: vec![],
        env,
        env_scripts: vec![],
        tasks,
        plugins: None,
    };

    check(
        &config,
        "markdown",
        &None,
        None,
        false,
        expect![[r#"
        #### No Category

        * **1** - 1
        * **2** - 2

    "#]],
    );
}

#[test]
fn run_all_public_markdown_sub_section() {
    let config_section = ConfigSection::new();
    let env = IndexMap::<String, EnvValue>::new();

    let mut tasks = IndexMap::<String, Task>::new();
    let mut task1 = Task::new();
    task1.description = Some("1".to_string());
    tasks.insert("1".to_string(), task1);
    let mut task2 = Task::new();
    task2.description = Some("2".to_string());
    tasks.insert("2".to_string(), task2);

    let config = Config {
        config: config_section,
        env_files: vec![],
        env,
        env_scripts: vec![],
        tasks,
        plugins: None,
    };

    check(
        &config,
        "markdown-sub-section",
        &None,
        None,
        false,
        expect![[r#"
            #### No Category

            * **1** - 1
            * **2** - 2

        "#]],
    );
}

#[test]
fn run_all_public_markdown_single_page() {
    let config_section = ConfigSection::new();
    let env = IndexMap::<String, EnvValue>::new();

    let mut tasks = IndexMap::<String, Task>::new();
    let mut task1 = Task::new();
    task1.description = Some("1".to_string());
    tasks.insert("1".to_string(), task1);
    let mut task2 = Task::new();
    task2.description = Some("2".to_string());
    tasks.insert("2".to_string(), task2);

    let config = Config {
        config: config_section,
        env_files: vec![],
        env,
        env_scripts: vec![],
        tasks,
        plugins: None,
    };

    check(
        &config,
        "markdown-single-page",
        &None,
        None,
        false,
        expect![[r#"
            # Task List

            ## No Category

            * **1** - 1
            * **2** - 2

        "#]],
    );
}

#[test]
fn run_all_private() {
    let config_section = ConfigSection::new();
    let env = IndexMap::<String, EnvValue>::new();

    let mut tasks = IndexMap::<String, Task>::new();
    let mut task1 = Task::new();
    task1.description = Some("1".to_string());
    task1.private = Some(true);
    tasks.insert("1".to_string(), task1);
    let mut task2 = Task::new();
    task2.description = Some("2".to_string());
    task2.private = Some(true);
    tasks.insert("2".to_string(), task2);

    let config = Config {
        config: config_section,
        env_files: vec![],
        env,
        env_scripts: vec![],
        tasks,
        plugins: None,
    };

    check(&config, "default", &None, None, false, expect![[""]]);
}

#[test]
fn run_mixed() {
    let config_section = ConfigSection::new();
    let env = IndexMap::<String, EnvValue>::new();

    let mut tasks = IndexMap::<String, Task>::new();
    let mut task1 = Task::new();
    task1.description = Some("1".to_string());
    task1.private = Some(true);
    tasks.insert("1".to_string(), task1);
    let mut task2 = Task::new();
    task2.description = Some("2".to_string());
    tasks.insert("2".to_string(), task2);
    let mut task3 = Task::new();
    task3.description = Some("3".to_string());
    task3.deprecated = Some(DeprecationInfo::Boolean(true));
    tasks.insert("3".to_string(), task3);
    let mut task4 = Task::new();
    task4.description = Some("4".to_string());
    task4.deprecated = Some(DeprecationInfo::Message("test".to_string()));
    tasks.insert("4".to_string(), task4);

    let config = Config {
        config: config_section,
        env_files: vec![],
        env,
        env_scripts: vec![],
        tasks,
        plugins: None,
    };

    check(
        &config,
        "default",
        &None,
        None,
        false,
        expect![[r#"
        No Category
        ----------
        2 - 2
        3 - 3 (deprecated)
        4 - 4 (deprecated - test)

    "#]],
    );
}

#[test]
fn run_write_to_file() {
    let config_section = ConfigSection::new();
    let env = IndexMap::<String, EnvValue>::new();

    let mut tasks = IndexMap::<String, Task>::new();
    let mut task1 = Task::new();
    task1.description = Some("1".to_string());
    tasks.insert("1".to_string(), task1);
    let mut task2 = Task::new();
    task2.description = Some("2".to_string());
    tasks.insert("2".to_string(), task2);

    let config = Config {
        config: config_section,
        env_files: vec![],
        env,
        env_scripts: vec![],
        tasks,
        plugins: None,
    };

    let file = "./target/_temp/tasklist.md";
    check(
        &config,
        "markdown-single-page",
        &Some(file.to_string()),
        None,
        false,
        expect![[r#"
            # Task List

            ## No Category

            * **1** - 1
            * **2** - 2

        "#]],
    );
}

#[test]
fn run_category_public() {
    let config_section = ConfigSection::new();
    let env = IndexMap::<String, EnvValue>::new();

    let mut tasks = IndexMap::<String, Task>::new();
    let mut task1 = Task::new();
    task1.description = Some("1".to_string());
    task1.category = Some("TestCategory1".to_string());
    tasks.insert("1".to_string(), task1);
    let mut task2 = Task::new();
    task2.description = Some("2".to_string());
    task2.category = Some("TestCategory1".to_string());
    tasks.insert("2".to_string(), task2);
    let mut task3 = Task::new();
    task3.description = Some("3".to_string());
    task3.category = Some("TestCategory2".to_string());
    tasks.insert("3".to_string(), task3);

    let config = Config {
        config: config_section,
        env_files: vec![],
        env,
        env_scripts: vec![],
        tasks,
        plugins: None,
    };

    check(
        &config,
        "default",
        &None,
        Some("TestCategory1".to_owned()),
        false,
        expect![[r#"
            TestCategory1
            ----------
            1 - 1
            2 - 2

        "#]],
    );
}
