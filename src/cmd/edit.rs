use std::ffi::OsStr;

use crate::{editor, specs::Specs};

use anyhow::Result;

/// edit subcommand entrypoint, will attempt to open the spec in the user's $EDITOR.
pub fn edit(specs: &Specs, name: &OsStr) -> Result<()> {
    let path = specs.get_spec_path(name)?;
    editor::start(&path)
}
