use anyhow::{Context, Result};
use minijinja::Environment;
use std::{
    ffi::OsStr,
    fs::{create_dir_all, write},
    path::PathBuf,
};

use crate::specs::{Spec, Specs};

/// generate corresponds to the gen subcommand. It generates the given template spec
pub fn generate(specs: &Specs, name: &OsStr, options: Vec<String>) -> Result<()> {
    let spec: Spec = specs
        .get_spec(name)
        .context("Unable to parse template file")?;

    // Copy the user's variable definitions in the spec as defaults, and override any of them if
    // specified as an option to the gen command. It attempts to convert to a toml::Value if
    // possible, and as a string if it can't.
    let mut variables = spec.variables.clone();
    for chunk in options.chunks(2) {
        if chunk.len() == 2 {
            let key = &chunk[0];
            let value = &chunk[1];
            let toml_value = toml::from_str::<toml::Value>(value)
                .unwrap_or_else(|_| toml::Value::String(value.clone()));
            variables.insert(key.clone(), toml_value);
        }
    }

    // Converts the template entries in the spec to a format acceptable to minijinja add_template
    // function. Environment::add_template expects 2 &str that must live for the lifetime of its
    // instance, so all this block is doing is converting the templates' PathBuf into something
    // that lives for the lifetime of env below
    let templates: Vec<(PathBuf, String, &str)> = spec
        .templates
        .iter()
        .map(|t| -> Result<(PathBuf, String, &str)> {
            let name = t
                .path
                .to_str()
                .ok_or_else(|| anyhow::anyhow!("Invalid UTF-8 in template path"))?
                .to_string();

            // to_path_buf to copy it, basically. Not ideal here
            Ok((t.path.to_path_buf(), name, t.body.as_ref()))
        })
        .collect::<Result<Vec<_>>>()?;

    // minijinja
    let mut env = Environment::new();

    for (path, name, template) in &templates {
        env.add_template(name, template)?;
        let render = env.get_template(name)?.render(&variables)?;

        if let Some(parent) = path.parent() {
            create_dir_all(parent)?;
        }

        write(path, render)?;
        println!("{}", name);
    }

    Ok(())
}
