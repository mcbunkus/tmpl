use std::{env, ffi::OsStr};

use anyhow::Result;

use crate::{
    editor,
    specs::{Spec, Specs, Template},
};

pub fn default_spec() -> Spec {
    let mut spec = Spec {
        variables: toml::Table::new(),
        templates: Vec::new(),
    };

    // this isn't super important, it's just for the sake of example
    let username = env::var("USER") // Unix/Linux/macOS
        .or_else(|_| env::var("USERNAME")) // Windows
        .unwrap_or_else(|_| "username".to_string());

    spec.variables
        .insert("user".into(), toml::Value::String(username));

    spec.variables
        .insert("project".into(), toml::Value::String("project-name".into()));

    spec.templates.push(Template {
        path: "README.md".into(),
        body: "
# {{ project }}

Created by {{ user }}.
"
        .into(),
    });

    spec
}

/// Generates a new, blank spec in the spec directory.
pub fn new(specs: &Specs, name: &OsStr, edit: bool) -> Result<()> {
    let spec = default_spec();
    specs.write_spec(name, &spec)?;
    println!("Created {}", name.display());

    if edit {
        let path = specs.safe_get_spec_path(name)?;
        return editor::start(&path);
    }

    Ok(())
}
