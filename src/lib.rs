mod editor;
mod prompt;

pub mod cli;
pub mod cmd;
pub mod io;
pub mod path;
pub mod specs;

use anyhow::Result;
use std::path::Path;

use crate::{io::IO, specs::Specs};

pub fn run(cli: cli::Cli, spec_dir: &Path) -> Result<()> {
    let specs = Specs::new(spec_dir)?;

    let mut io = IO::default();

    match cli.command {
        cli::Commands::Ls(args) => cmd::list(&specs, args, &mut io)?,
        cli::Commands::New(args) => cmd::new(&specs, args, &mut io)?,
        cli::Commands::Gen(args) => cmd::generate(&specs, args, &mut io)?,
        cli::Commands::Edit(args) => cmd::edit(&specs, args)?,
        cli::Commands::Cp(args) => cmd::cp(&specs, args)?,
        cli::Commands::Rm(args) => cmd::rm(&specs, args, &mut io)?,
    }

    Ok(())
}
