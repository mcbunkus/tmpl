use anyhow::{Context, Result};
use std::fs::{self, DirEntry};

use crate::specs::{Spec, Specs};

fn list_without_vars(specs: Vec<DirEntry>) -> Result<()> {
    for spec in specs {
        println!("{}", spec.file_name().display());
    }

    Ok(())
}

fn list_with_vars(specs: Vec<DirEntry>) -> Result<()> {
    // I'm sure there's bugs galore in this section
    let max_col_len = specs
        .iter()
        .map(|e| {
            if let Ok(s) = e.file_name().into_string() {
                s.len()
            } else {
                e.file_name().len()
            }
        })
        .max()
        .unwrap_or(0);

    for entry in specs {
        let contents = fs::read_to_string(entry.path()).context("Failed to open for reading")?;
        let spec: Spec = toml::from_str(&contents).context("Failed to parse spec file contents")?;

        let vars = spec
            .variables
            .iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect::<Vec<_>>()
            .join(", ");

        let name = entry.file_name();
        println!("{:<width$}\t{}", name.display(), vars, width = max_col_len);
    }

    Ok(())
}

pub fn list(specs: &Specs, list_vars: bool) -> Result<()> {
    // entries is a list of regular files in the spec directory. This block filters out
    // anything that's not a regular file.
    let specs: Vec<DirEntry> = fs::read_dir(specs.dir())?
        .filter_map(|e| {
            if let Ok(entry) = e
                && let Ok(file_type) = entry.file_type()
                && file_type.is_file()
            {
                Some(entry)
            } else {
                None
            }
        })
        .collect();

    if specs.is_empty() {
        println!(
            "You don't have any templates yet. Please create a new one with: tmpl new <name of your template>"
        );
        return Ok(());
    }

    // These functions assume everything in the entries vector is a regular file it can read. No
    // directories.
    if list_vars {
        list_with_vars(specs)
    } else {
        list_without_vars(specs)
    }
}
