use anyhow::{Context, Result};

use crate::{CpArgs, prompt::prompt_yn, specs::Specs};

pub fn cp(specs: &Specs, args: CpArgs) -> Result<()> {
    if specs.exists(&args.dest) && !args.skip_prompt {
        let question = format!(
            "{} exists, do you want to overwrite it?",
            args.dest.display()
        );

        let overwrite = prompt_yn(&question, false).context("Do you wish to overwrite prompt")?;

        if !overwrite {
            return Ok(());
        }
    }

    specs.copy(&args.source, &args.dest)
}
