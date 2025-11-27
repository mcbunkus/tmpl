use anyhow::Result;
use std::ffi::{OsStr, OsString};

use crate::{prompt::prompt_yn, specs::Specs};

/// Delete a spec from the spec directory. Errors aren't fatal in this module's context, so it's
/// just logged in this function.
fn remove(specs: &Specs, name: &OsStr) {
    let res = specs.delete_spec(name);

    // We want to continue deleting other specs, even if this one failed, therefore errors aren't bubbled
    // up here
    if res.is_err()
        && let Some(err) = res.err()
    {
        println!("Failed to delete spec: {:#}", err)
    } else {
        println!("Deleted {}", name.display());
    }
}

/// Loop through all the specs the user wants to delete and delete them.
fn remove_without_prompt(specs: &Specs, to_delete: Vec<OsString>) {
    for candidate in to_delete {
        remove(specs, &candidate);
    }
}

/// Loop through all the specs, and delete them if the user confirms that yes, they did in fact
/// want to delete it.
fn remove_with_prompt(specs: &Specs, to_delete: Vec<OsString>) {
    for candidate in to_delete {
        let question = format!("Remove {}", candidate.display());

        match prompt_yn(&question, false) {
            Ok(delete) => {
                if !delete {
                    println!("Skipping {}", candidate.display());
                    continue;
                }
                remove(specs, &candidate);
            }
            Err(err) => {
                println!("Unexpected error handling prompt: {:#}", err);
            }
        }
    }
}

/// rm subcommand entry point that deletes specs from the spec directory.
pub fn rm(specs: &Specs, to_delete: Vec<OsString>, skip_prompt: bool) -> Result<()> {
    if skip_prompt {
        remove_without_prompt(specs, to_delete);
    } else {
        remove_with_prompt(specs, to_delete);
    }

    Ok(())
}
