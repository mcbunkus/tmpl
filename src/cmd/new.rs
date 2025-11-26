use std::{collections::HashMap, env, ffi::OsStr};

use anyhow::Result;

use crate::{
    editor,
    specs::{Spec, Specs, Template},
};

mod example {
    pub const README_BODY: &str = "
# {{ project }}

Created by {{ user }}.
";
}

pub fn new(specs: &Specs, name: &OsStr, edit: bool) -> Result<()> {
    let mut spec = Spec {
        variables: HashMap::new(),
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
        body: example::README_BODY.into(),
    });

    specs.write_spec(name, &spec)?;
    println!("Created {}", name.display());

    if edit {
        let path = specs.get_path(name)?;
        return editor::start(&path);
    }

    Ok(())
}
