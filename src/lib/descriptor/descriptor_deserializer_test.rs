use super::*;
use crate::descriptor::makefiles;

#[test]
fn load_config_base() {
    load_config(makefiles::BASE, true).unwrap();
}

#[test]
fn load_config_stable() {
    load_config(makefiles::STABLE, true).unwrap();
}

#[test]
fn load_config_beta() {
    load_config(makefiles::BETA, true).unwrap();
}

#[test]
#[should_panic]
fn load_config_invalid_validate() {
    load_config(
        r#"
env_files = []
env_scripts = []

[config]
init_task = "init"
end_task = "end"

[env]

[tasks.empty]
description = "Empty Task"
category2 = "Tools"
    "#,
        true,
    )
    .unwrap();
}

#[test]
fn load_config_invalid_no_validate() {
    let config = load_config(
        r#"
env_files = []
env_scripts = []

[config]
init_task = "init"
end_task = "end"

[env]

[tasks.empty]
description = "Empty Task"
category2 = "Tools"
    "#,
        false,
    )
    .unwrap();

    assert!(config.tasks.contains_key("empty"));
}

#[test]
fn load_external_config_warning() {
    let config = load_external_config(
        r#"
[tasks.empty]
description = "Empty Task"
category2 = "Tools"
    "#,
        "somefile",
    )
    .unwrap();

    assert!(config.tasks.unwrap().contains_key("empty"));
}
