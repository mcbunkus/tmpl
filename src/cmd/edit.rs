use crate::cli::EditArgs;
use crate::{editor, specs::Specs};

use anyhow::Result;

/// edit subcommand entrypoint, will attempt to open the spec in the user's $EDITOR.
pub fn edit(specs: &Specs, args: EditArgs) -> Result<()> {
    let path = specs.safe_get_spec_path(&args.name)?;
    editor::start(&path)
}
