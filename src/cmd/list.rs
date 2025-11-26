use anyhow::{Context, Result};
use std::{
    fs::{self, DirEntry},
    path::PathBuf,
};

use crate::template::Spec;

fn list_without_vars(entries: Vec<DirEntry>) -> Result<()> {
    for entry in entries {
        println!("{}", entry.file_name().display());
    }

    Ok(())
}

fn list_with_vars(entries: Vec<DirEntry>) -> Result<()> {
    // I'm sure there's bugs galore in this section
    let max_col_len = entries
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

    for entry in entries {
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

pub fn list(template_directory: PathBuf, list_vars: bool) -> Result<()> {
    // entries is a list of regular files in the templates directory. This block filters out
    // anything that's not a regular file.
    let entries: Vec<DirEntry> = fs::read_dir(&template_directory)?
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

    if entries.is_empty() {
        println!(
            "You don't have any templates yet. Please create a new one with: tmpl new <name of your template>"
        );
        return Ok(());
    }

    // These functions assume everything in the entries vector is a regular file it can read. No
    // directories.
    if list_vars {
        list_with_vars(entries)
    } else {
        list_without_vars(entries)
    }
}
