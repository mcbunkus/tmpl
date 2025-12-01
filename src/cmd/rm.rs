use anyhow::{Context, Result};
use std::ffi::{OsStr, OsString};

use crate::{prompt::prompt_yn, specs::Specs};

/// Delete a spec from the spec directory. Errors aren't fatal in this module's context, except
/// errors writing to stdout or stderr, so they're just logged in this function.
fn remove<W: std::io::Write, E: std::io::Write>(
    specs: &Specs,
    name: &OsStr,
    out: &mut W,
    err: &mut E,
) -> Result<()> {
    match specs.delete_spec(name) {
        Ok(_) => writeln!(out, "Deleted {}", name.display())
            .context("Failed to write to \"stdout\" writer"),
        Err(e) => writeln!(err, "Failed to delete spec: {:#}", e)
            .context("Failed to write to \"stderr\" writer"),
    }
}

/// Loop through all the specs the user wants to delete and delete them.
fn remove_without_prompt<W: std::io::Write, E: std::io::Write>(
    specs: &Specs,
    to_delete: Vec<OsString>,
    out: &mut W,
    err: &mut E,
) -> Result<()> {
    for candidate in to_delete {
        remove(specs, &candidate, out, err)?;
    }
    Ok(())
}

/// Loop through all the specs, and delete them if the user confirms that yes, they did in fact
/// want to delete it.
fn remove_with_prompt<W: std::io::Write, E: std::io::Write>(
    specs: &Specs,
    to_delete: Vec<OsString>,
    out: &mut W,
    err: &mut E,
) -> Result<()> {
    for candidate in to_delete {
        let question = format!("Remove {}", candidate.display());

        match prompt_yn(&question, false) {
            Ok(delete) => {
                if !delete {
                    writeln!(out, "Skipping {}", candidate.display())?;
                    continue;
                }
                remove(specs, &candidate, out, err)?;
            }
            Err(e) => {
                writeln!(err, "Unexpected error handling prompt: {:#}", e)?;
            }
        }
    }

    Ok(())
}

/// rm subcommand entry point that deletes specs from the spec directory.
pub fn rm<W: std::io::Write, E: std::io::Write>(
    specs: &Specs,
    to_delete: Vec<OsString>,
    skip_prompt: bool,
    out: &mut W,
    err: &mut E,
) -> Result<()> {
    if skip_prompt {
        remove_without_prompt(specs, to_delete, out, err)
    } else {
        remove_with_prompt(specs, to_delete, out, err)
    }
}
