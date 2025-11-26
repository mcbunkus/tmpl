use anyhow::{Context, Result};
use std::{path::Path, process::Command};

/// Spawn the user's editor, returning an error if the user didn't set it, or there was an issue
/// spawning it.
pub fn start(path: &Path) -> Result<()> {
    let editor = std::env::var_os("EDITOR").ok_or_else(|| anyhow::anyhow!("EDITOR environment variable is not set. Please set this environment variable to the path of your preferred text editor."))?;
    Command::new(&editor)
        .arg(path)
        .status()
        .context(format!(
            "Failed to launch editor (editor: {})",
            editor.display()
        ))?
        .success()
        .then_some(())
        .context("Editor exited with non-zero status")
}
