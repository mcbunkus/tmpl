use std::{collections::HashMap, env, ffi::OsString, fs, path::PathBuf, process::Command};

use anyhow::{Context, Result};

use crate::template::{Spec, Template};

pub fn new(template_directory: PathBuf, name: String, edit: bool, editor: OsString) -> Result<()> {
    let template_path = template_directory.join(&name);
    anyhow::ensure!(!template_path.is_file(), "{} already exists", name);

    let mut spec = Spec {
        variables: HashMap::new(),
        templates: Vec::new(),
    };

    // this isn't super important, it's just for the sake of example
    let username = env::var("USER") // Unix/Linux/macOS
        .or_else(|_| env::var("USERNAME")) // Windows
        .unwrap_or_else(|_| "awesome.user".to_string());

    spec.variables
        .insert("user".into(), toml::Value::String(username));

    spec.templates.push(Template {
        path: PathBuf::from("doc").join("README.md"),
        body: "Hello, {{ user }}!".into(),
    });

    let toml_string = toml::to_string(&spec)?;

    fs::write(&template_path, toml_string)?;
    println!("Created {}", name);

    if edit {
        Command::new(&editor)
            .arg(&template_path)
            .status()
            .context("Failed to launch editor")?
            .success()
            .then_some(())
            .context("Editor exited with non-zero status")?;
    }

    Ok(())
}
