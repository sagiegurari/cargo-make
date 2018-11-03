use super::*;

#[test]
fn install_empty() {
    let task = Task::new();

    install(&task);
}

#[test]
fn install_crate_already_installed() {
    let mut task = Task::new();
    task.install_crate = Some(InstallCrate::Value("test".to_string()));
    task.command = Some("cargo".to_string());
    task.args = Some(vec!["test".to_string()]);

    install(&task);
}

#[test]
#[should_panic]
fn install_crate_missing_cargo_command() {
    let mut task = Task::new();
    task.install_crate = Some(InstallCrate::Value("test".to_string()));
    task.command = Some("cargo".to_string());

    install(&task);
}

#[test]
fn install_crate_auto_detect_already_installed() {
    let mut task = Task::new();
    task.command = Some("cargo".to_string());
    task.args = Some(vec!["test".to_string()]);

    install(&task);
}

#[test]
#[should_panic]
fn install_crate_auto_detect_unable_to_install() {
    let mut task = Task::new();
    task.command = Some("cargo".to_string());
    task.args = Some(vec!["badbadbad".to_string()]);

    install(&task);
}

#[test]
fn install_script_ok() {
    let mut task = Task::new();
    task.install_script = Some(vec!["exit 0".to_string()]);

    install(&task);
}

#[test]
#[should_panic]
fn install_script_error() {
    let mut task = Task::new();
    task.install_script = Some(vec!["exit 1".to_string()]);

    install(&task);
}

#[test]
fn install_script_error_force() {
    let mut task = Task::new();
    task.force = Some(true);
    task.install_script = Some(vec!["exit 1".to_string()]);

    install(&task);
}
