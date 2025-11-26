use anyhow::Result;
use std::ffi::{OsStr, OsString};

use crate::{prompt::prompt_yn, specs::Specs};

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

fn remove_without_prompt(specs: &Specs, to_delete: Vec<OsString>) {
    for candidate in to_delete {
        remove(specs, &candidate);
    }
}

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

pub fn rm(specs: &Specs, to_delete: Vec<OsString>, skip_prompt: bool) -> Result<()> {
    if skip_prompt {
        remove_without_prompt(specs, to_delete);
    } else {
        remove_with_prompt(specs, to_delete);
    }

    Ok(())
}
