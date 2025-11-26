use std::ffi::OsStr;

use crate::{editor, specs::Specs};

use anyhow::Result;

pub fn edit(specs: &Specs, name: &OsStr) -> Result<()> {
    let path = specs.get_path(name)?;
    editor::start(&path)
}
