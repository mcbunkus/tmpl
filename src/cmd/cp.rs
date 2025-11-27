use anyhow::{Context, Result};
use std::ffi::OsStr;

use crate::{prompt::prompt_yn, specs::Specs};

pub fn cp(specs: &Specs, source: &OsStr, dest: &OsStr, skip_prompt: bool) -> Result<()> {
    if specs.exists(dest) && !skip_prompt {
        let question = format!("{} exists, do you want to overwrite it?", dest.display());

        let overwrite = prompt_yn(&question, false).context("Do you wish to overwrite prompt")?;

        if !overwrite {
            return Ok(());
        }
    }

    specs.copy(source, dest)
}
