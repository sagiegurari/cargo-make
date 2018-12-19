use super::*;
use crate::types::{ConfigSection, Task};
use indexmap::IndexMap;

#[test]
fn print_default() {
    let mut config = Config {
        config: ConfigSection::new(),
        env: IndexMap::new(),
        tasks: IndexMap::new(),
    };

    config.tasks.insert("init".to_string(), Task::new());
    config.tasks.insert("end".to_string(), Task::new());
    config.tasks.insert("test".to_string(), Task::new());

    print(&config, "test", false);
}

#[test]
#[should_panic]
fn print_task_not_found() {
    let mut config = Config {
        config: ConfigSection::new(),
        env: IndexMap::new(),
        tasks: IndexMap::new(),
    };

    config.tasks.insert("init".to_string(), Task::new());
    config.tasks.insert("end".to_string(), Task::new());

    print(&config, "test", false);
}
