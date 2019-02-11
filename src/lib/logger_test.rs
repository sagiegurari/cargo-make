use super::*;

#[test]
fn get_level_verbose() {
    let level = get_level("verbose");
    assert_eq!(level, LogLevel::VERBOSE);
}

#[test]
fn get_level_info() {
    let level = get_level("info");
    assert_eq!(level, LogLevel::INFO);
}

#[test]
fn get_level_error() {
    let level = get_level("error");
    assert_eq!(level, LogLevel::ERROR);
}

#[test]
fn get_level_other() {
    let level = get_level("test123");
    assert_eq!(level, LogLevel::INFO);
}

#[test]
fn get_name_for_level_verbose() {
    let level = get_name_for_level(&Level::Debug);
    assert_eq!(level, "verbose");
}

#[test]
fn get_name_for_level_info() {
    let level = get_name_for_level(&Level::Info);
    assert_eq!(level, "info");
}

#[test]
fn get_name_for_level_error() {
    let level = get_name_for_level(&Level::Error);
    assert_eq!(level, "error");
}

#[test]
fn get_name_for_level_warn() {
    let level = get_name_for_level(&Level::Warn);
    assert_eq!(level, "warn");
}

#[test]
fn get_name_for_level_other() {
    let level = get_name_for_level(&Level::Trace);
    assert_eq!(level, "info");
}

#[test]
fn get_name_for_filter_verbose() {
    let level = get_name_for_filter(&LevelFilter::Debug);
    assert_eq!(level, "verbose");
}

#[test]
fn get_name_for_filter_info() {
    let level = get_name_for_filter(&LevelFilter::Info);
    assert_eq!(level, "info");
}

#[test]
fn get_name_for_filter_error() {
    let level = get_name_for_filter(&LevelFilter::Error);
    assert_eq!(level, "error");
}

#[test]
fn get_name_for_filter_warn() {
    let level = get_name_for_filter(&LevelFilter::Warn);
    assert_eq!(level, "warn");
}

#[test]
fn get_name_for_filter_other() {
    let level = get_name_for_filter(&LevelFilter::Trace);
    assert_eq!(level, "info");
}

#[test]
fn get_formatted_name_with_color() {
    let output = get_formatted_name("test", true);

    assert_eq!("test".bold(), output);
}

#[test]
fn get_formatted_name_no_color() {
    let output = get_formatted_name("test", false);

    assert_eq!("test".normal(), output);
}

#[test]
fn get_formatted_log_level_debug_with_color() {
    let output = get_formatted_log_level(&Level::Debug, true);

    assert_eq!("verbose".cyan().bold(), output);
}

#[test]
fn get_formatted_log_level_info_with_color() {
    let output = get_formatted_log_level(&Level::Info, true);

    assert_eq!("info".bold(), output);
}

#[test]
fn get_formatted_log_level_warn_with_color() {
    let output = get_formatted_log_level(&Level::Warn, true);

    assert_eq!("warn".yellow().bold(), output);
}

#[test]
fn get_formatted_log_level_error_with_color() {
    let output = get_formatted_log_level(&Level::Error, true);

    assert_eq!("error".red().bold(), output);
}

#[test]
fn get_formatted_log_level_debug_no_color() {
    let output = get_formatted_log_level(&Level::Debug, false);

    assert_eq!("verbose".normal(), output);
}

#[test]
fn get_formatted_log_level_info_no_color() {
    let output = get_formatted_log_level(&Level::Info, false);

    assert_eq!("info".normal(), output);
}

#[test]
fn get_formatted_log_level_warn_no_color() {
    let output = get_formatted_log_level(&Level::Warn, false);

    assert_eq!("warn".normal(), output);
}

#[test]
fn get_formatted_log_level_error_no_color() {
    let output = get_formatted_log_level(&Level::Error, false);

    assert_eq!("error".normal(), output);
}

#[test]
#[should_panic]
fn exit_test() {
    exit(0);
}

#[test]
#[should_panic]
fn create_error() {
    init(&LoggerOptions {
        level: "error".to_string(),
        color: false,
    });

    error!("test");
}
