use anyhow::{Context, Result};
use std::{
    ffi::{OsStr, OsString},
    io::Write,
};

use crate::{RmArgs, io::IO, prompt::prompt_yn, specs::Specs};

/// Delete a spec from the spec directory. Errors aren't fatal in this module's context, except
/// errors writing to stdout or stderr, so they're just logged in this function.
fn remove<Stdout: Write, Stderr: Write>(
    specs: &Specs,
    name: &OsStr,
    io: &mut IO<Stdout, Stderr>,
) -> Result<()> {
    match specs.delete_spec(name) {
        Ok(_) => writeln!(io.stdout(), "Deleted {}", name.display())
            .context("Failed to write to \"stdout\" writer"),
        Err(e) => writeln!(io.stderr(), "Failed to delete spec: {:#}", e)
            .context("Failed to write to \"stderr\" writer"),
    }
}

/// Loop through all the specs the user wants to delete and delete them.
fn remove_without_prompt<Stdout: Write, Stderr: Write>(
    specs: &Specs,
    to_delete: Vec<OsString>,
    io: &mut IO<Stdout, Stderr>,
) -> Result<()> {
    for candidate in to_delete {
        remove(specs, &candidate, io)?;
    }
    Ok(())
}

/// Loop through all the specs, and delete them if the user confirms that yes, they did in fact
/// want to delete it.
fn remove_with_prompt<Stdout: Write, Stderr: Write>(
    specs: &Specs,
    to_delete: Vec<OsString>,
    io: &mut IO<Stdout, Stderr>,
) -> Result<()> {
    for candidate in to_delete {
        let question = format!("Remove {}", candidate.display());

        match prompt_yn(&question, false) {
            Ok(delete) => {
                if !delete {
                    writeln!(io.stdout(), "Skipping {}", candidate.display())?;
                    continue;
                }
                remove(specs, &candidate, io)?;
            }
            Err(e) => {
                writeln!(io.stderr(), "Unexpected error handling prompt: {:#}", e)?;
            }
        }
    }

    Ok(())
}

/// rm subcommand entry point that deletes specs from the spec directory.
pub fn rm<Stdout: Write, Stderr: Write>(
    specs: &Specs,
    args: RmArgs,
    io: &mut IO<Stdout, Stderr>,
) -> Result<()> {
    if args.skip_prompt {
        remove_without_prompt(specs, args.to_delete, io)
    } else {
        remove_with_prompt(specs, args.to_delete, io)
    }
}
