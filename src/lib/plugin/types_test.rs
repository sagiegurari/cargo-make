use super::*;

#[test]
fn plugins_new() {
    let plugins = Plugins::new();

    assert!(plugins.aliases.is_none());
    assert!(plugins.plugins.is_empty());
}
