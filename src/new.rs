use std::{collections::HashMap, fs, path::PathBuf};

use anyhow::Result;

use crate::template::Spec;

pub fn new(template_directory: PathBuf, name: String) -> Result<()> {
    let template_path = template_directory.join(&name);
    anyhow::ensure!(!template_path.is_file(), "{} already exists", name);

    let spec = Spec {
        templates: Vec::new(),
        variables: HashMap::new(),
    };

    let toml_string = toml::to_string_pretty(&spec)?;

    fs::write(&template_path, toml_string)?;
    println!("Created {}", name);

    Ok(())
}
