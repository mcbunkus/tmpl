use anyhow::Result;
use std::ffi::OsString;
use unicode_width::UnicodeWidthStr;

use crate::specs::{Spec, Specs};

/// Simply list the name of all the specs in the spec directory.
fn list_without_vars<W: std::io::Write>(specs: Vec<OsString>, writer: &mut W) -> Result<()> {
    for spec in specs {
        writeln!(writer, "{}", spec.display())?;
    }

    Ok(())
}

/// List all the specs in the specs directory, including their default variables.
fn list_with_vars<W: std::io::Write>(
    specs: &Specs,
    names: Vec<OsString>,
    writer: &mut W,
) -> Result<()> {
    let max_col_len = names
        .iter()
        .filter_map(|os| {
            if let Ok(s) = os.clone().into_string() {
                Some(s.width())
            } else {
                None
            }
        })
        .max()
        .unwrap_or(0);

    for name in names {
        let spec: Spec = specs.read_spec(&name)?;

        let vars = spec
            .variables
            .iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect::<Vec<_>>()
            .join(", ");

        writeln!(
            writer,
            "{:<width$}\t{}",
            name.display(),
            vars,
            width = max_col_len
        )?;
    }

    Ok(())
}

/// ls subcommand entrypoint. It takes and writes to a writer instead of stdout for unit testing
/// purposes.
pub fn list<W: std::io::Write>(specs: &Specs, list_vars: bool, writer: &mut W) -> Result<()> {
    let all_specs = specs.get_all_specs()?;

    if all_specs.is_empty() {
        writeln!(
            writer,
            "You don't have any templates yet. Please create a new one with: tmpl new <name of your template>"
        )?;
        return Ok(());
    }

    // These functions assume everything in the entries vector is a regular file it can read. No
    // directories.
    if list_vars {
        list_with_vars(specs, all_specs, writer)
    } else {
        list_without_vars(all_specs, writer)
    }
}
