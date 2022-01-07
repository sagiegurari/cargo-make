use super::*;

#[test]
fn merge_aliases_empty() {
    let base = IndexMap::new();
    let extended = IndexMap::new();

    let output = merge_aliases(&base, &extended);

    assert!(output.is_empty());
}

#[test]
fn merge_aliases_base_only() {
    let mut base = IndexMap::new();
    base.insert("old".to_string(), "new".to_string());
    let extended = IndexMap::new();

    let output = merge_aliases(&base, &extended);

    assert_eq!(output.len(), 1);
    assert_eq!(output.get("old").unwrap(), "new");
}

#[test]
fn merge_aliases_extended_only() {
    let base = IndexMap::new();
    let mut extended = IndexMap::new();
    extended.insert("old".to_string(), "new".to_string());

    let output = merge_aliases(&base, &extended);

    assert_eq!(output.len(), 1);
    assert_eq!(output.get("old").unwrap(), "new");
}

#[test]
fn merge_aliases_both_and_duplicates() {
    let mut base = IndexMap::new();
    base.insert("base".to_string(), "base2".to_string());
    base.insert("test".to_string(), "base".to_string());
    let mut extended = IndexMap::new();
    extended.insert("extended".to_string(), "extended2".to_string());
    extended.insert("test".to_string(), "extended".to_string());

    let output = merge_aliases(&base, &extended);

    assert_eq!(output.len(), 3);
    assert_eq!(output.get("base").unwrap(), "base2");
    assert_eq!(output.get("extended").unwrap(), "extended2");
    assert_eq!(output.get("test").unwrap(), "extended");
}

#[test]
fn merge_plugins_map_empty() {
    let base = IndexMap::new();
    let extended = IndexMap::new();

    let output = merge_plugins_map(&base, &extended);

    assert!(output.is_empty());
}

#[test]
fn merge_plugins_map_base_only() {
    let mut base = IndexMap::new();
    base.insert(
        "plugin".to_string(),
        Plugin {
            script: "test".to_string(),
        },
    );
    let extended = IndexMap::new();

    let output = merge_plugins_map(&base, &extended);

    assert_eq!(output.len(), 1);
    assert_eq!(output.get("plugin").unwrap().script, "test");
}

#[test]
fn merge_plugins_map_extended_only() {
    let base = IndexMap::new();
    let mut extended = IndexMap::new();
    extended.insert(
        "plugin".to_string(),
        Plugin {
            script: "test".to_string(),
        },
    );

    let output = merge_plugins_map(&base, &extended);

    assert_eq!(output.len(), 1);
    assert_eq!(output.get("plugin").unwrap().script, "test");
}

#[test]
fn merge_plugins_map_both_and_duplicates() {
    let mut base = IndexMap::new();
    base.insert(
        "base".to_string(),
        Plugin {
            script: "base".to_string(),
        },
    );
    base.insert(
        "test".to_string(),
        Plugin {
            script: "test1".to_string(),
        },
    );
    let mut extended = IndexMap::new();
    extended.insert(
        "extended".to_string(),
        Plugin {
            script: "extended".to_string(),
        },
    );
    extended.insert(
        "test".to_string(),
        Plugin {
            script: "test2".to_string(),
        },
    );

    let output = merge_plugins_map(&base, &extended);

    assert_eq!(output.len(), 3);
    assert_eq!(output.get("base").unwrap().script, "base");
    assert_eq!(output.get("extended").unwrap().script, "extended");
    assert_eq!(output.get("test").unwrap().script, "test2");
}

#[test]
fn merge_plugins_config_impl_aliases_none() {
    let base = Plugins::new();
    let extended = Plugins::new();

    let output = merge_plugins_config_impl(base, extended);

    assert!(output.aliases.is_none());
    assert!(output.plugins.is_empty());
}

#[test]
fn merge_plugins_config_impl_extended_aliases_none() {
    let mut base = Plugins::new();
    let mut aliases = IndexMap::new();
    aliases.insert("old".to_string(), "new".to_string());
    base.aliases = Some(aliases);
    let extended = Plugins::new();

    let output = merge_plugins_config_impl(base, extended);

    assert!(output.aliases.is_some());
    assert_eq!(output.aliases.unwrap().get("old").unwrap(), "new");
    assert!(output.plugins.is_empty());
}

#[test]
fn merge_plugins_config_impl_base_aliases_none() {
    let base = Plugins::new();
    let mut extended = Plugins::new();
    let mut aliases = IndexMap::new();
    aliases.insert("old".to_string(), "new".to_string());
    extended.aliases = Some(aliases);

    let output = merge_plugins_config_impl(base, extended);

    assert!(output.aliases.is_some());
    assert_eq!(output.aliases.unwrap().get("old").unwrap(), "new");
    assert!(output.plugins.is_empty());
}

#[test]
fn merge_plugins_config_impl_merge_all() {
    let mut base = Plugins::new();
    let mut extended = Plugins::new();

    let mut aliases = IndexMap::new();
    aliases.insert("base".to_string(), "basenew".to_string());
    aliases.insert("test".to_string(), "base".to_string());
    base.aliases = Some(aliases);

    aliases = IndexMap::new();
    aliases.insert("extended".to_string(), "extendednew".to_string());
    aliases.insert("test".to_string(), "extended".to_string());
    extended.aliases = Some(aliases);

    let mut plugins = IndexMap::new();
    plugins.insert(
        "base".to_string(),
        Plugin {
            script: "base".to_string(),
        },
    );
    plugins.insert(
        "test".to_string(),
        Plugin {
            script: "test1".to_string(),
        },
    );
    base.plugins = plugins;

    plugins = IndexMap::new();
    plugins.insert(
        "extended".to_string(),
        Plugin {
            script: "extended".to_string(),
        },
    );
    plugins.insert(
        "test".to_string(),
        Plugin {
            script: "test2".to_string(),
        },
    );
    extended.plugins = plugins;

    let output = merge_plugins_config_impl(base, extended);

    aliases = output.aliases.unwrap();
    assert_eq!(aliases.len(), 3);
    assert_eq!(aliases.get("base").unwrap(), "basenew");
    assert_eq!(aliases.get("extended").unwrap(), "extendednew");
    assert_eq!(aliases.get("test").unwrap(), "extended");
    assert_eq!(output.plugins.len(), 3);
    assert_eq!(output.plugins.get("base").unwrap().script, "base");
    assert_eq!(output.plugins.get("extended").unwrap().script, "extended");
    assert_eq!(output.plugins.get("test").unwrap().script, "test2");
}

#[test]
fn merge_plugins_config_none() {
    let output = merge_plugins_config(None, None);

    assert!(output.is_none());
}

#[test]
fn merge_plugins_config_extended_none() {
    let mut base = Plugins::new();

    let mut aliases = IndexMap::new();
    aliases.insert("test".to_string(), "test".to_string());
    base.aliases = Some(aliases);

    let mut plugins = IndexMap::new();
    plugins.insert(
        "test".to_string(),
        Plugin {
            script: "test1".to_string(),
        },
    );
    base.plugins = plugins;

    let output = merge_plugins_config(Some(base), None);

    assert!(output.is_some());

    let plugins_wrapper = output.unwrap();

    aliases = plugins_wrapper.aliases.unwrap();
    assert_eq!(aliases.len(), 1);
    assert_eq!(aliases.get("test").unwrap(), "test");
    assert_eq!(plugins_wrapper.plugins.len(), 1);
    assert_eq!(plugins_wrapper.plugins.get("test").unwrap().script, "test1");
}

#[test]
fn merge_plugins_config_base_none() {
    let mut extended = Plugins::new();

    let mut aliases = IndexMap::new();
    aliases.insert("test".to_string(), "test".to_string());
    extended.aliases = Some(aliases);

    let mut plugins = IndexMap::new();
    plugins.insert(
        "test".to_string(),
        Plugin {
            script: "test1".to_string(),
        },
    );
    extended.plugins = plugins;

    let output = merge_plugins_config(None, Some(extended));

    assert!(output.is_some());

    let plugins_wrapper = output.unwrap();

    aliases = plugins_wrapper.aliases.unwrap();
    assert_eq!(aliases.len(), 1);
    assert_eq!(aliases.get("test").unwrap(), "test");
    assert_eq!(plugins_wrapper.plugins.len(), 1);
    assert_eq!(plugins_wrapper.plugins.get("test").unwrap().script, "test1");
}

#[test]
fn merge_plugins_config_both_provided() {
    let mut base = Plugins::new();
    let mut extended = Plugins::new();

    let mut aliases = IndexMap::new();
    aliases.insert("base".to_string(), "1".to_string());
    aliases.insert("test".to_string(), "1".to_string());
    base.aliases = Some(aliases);
    aliases = IndexMap::new();
    aliases.insert("extended".to_string(), "1".to_string());
    aliases.insert("test".to_string(), "2".to_string());
    extended.aliases = Some(aliases);

    let mut plugins = IndexMap::new();
    plugins.insert(
        "base".to_string(),
        Plugin {
            script: "1".to_string(),
        },
    );
    plugins.insert(
        "test".to_string(),
        Plugin {
            script: "1".to_string(),
        },
    );
    base.plugins = plugins;
    plugins = IndexMap::new();
    plugins.insert(
        "extended".to_string(),
        Plugin {
            script: "1".to_string(),
        },
    );
    plugins.insert(
        "test".to_string(),
        Plugin {
            script: "2".to_string(),
        },
    );
    extended.plugins = plugins;

    let output = merge_plugins_config(Some(base), Some(extended));

    assert!(output.is_some());

    let plugins_wrapper = output.unwrap();

    aliases = plugins_wrapper.aliases.unwrap();
    assert_eq!(aliases.len(), 3);
    assert_eq!(aliases.get("base").unwrap(), "1");
    assert_eq!(aliases.get("extended").unwrap(), "1");
    assert_eq!(aliases.get("test").unwrap(), "2");
    assert_eq!(plugins_wrapper.plugins.len(), 3);
    assert_eq!(plugins_wrapper.plugins.get("base").unwrap().script, "1");
    assert_eq!(plugins_wrapper.plugins.get("extended").unwrap().script, "1");
    assert_eq!(plugins_wrapper.plugins.get("test").unwrap().script, "2");
}
