use std::{ffi::OsString, path::PathBuf, process::Command};

use anyhow::{Context, Result};

pub fn edit(template_directory: PathBuf, name: String, editor: OsString) -> Result<()> {
    let template_path = template_directory.join(&name);

    anyhow::ensure!(template_path.exists(), "{} does not exist", name);
    anyhow::ensure!(template_path.is_file(), "{} is not a file", name);

    Command::new(&editor)
        .arg(&template_path)
        .status()
        .context("Failed to launch editor")?
        .success()
        .then_some(())
        .context("Editor exited with non-zero status")
}
