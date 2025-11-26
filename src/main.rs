mod cli;
mod cmd;
mod editor;
mod prompt;
mod specs;

use anyhow::Result;
use clap::Parser;
use directories::ProjectDirs;
use std::fs;

use crate::{
    cli::{Cli, Commands},
    specs::Specs,
};

fn main() -> Result<()> {
    let cli = Cli::parse();

    let spec_dir = ProjectDirs::from("org", "mcbunkus", "tmpl")
        .ok_or_else(|| anyhow::anyhow!("Unable to find base directories"))?
        .data_dir()
        .to_path_buf();

    fs::create_dir_all(&spec_dir)?;

    let specs = Specs::new(&spec_dir)?;

    match cli.command {
        Commands::Ls { list_vars } => cmd::list(&specs, list_vars)?,
        Commands::New { name, edit } => cmd::new(&specs, &name, edit)?,
        Commands::Gen { name, options } => cmd::generate(&specs, &name, options)?,
        Commands::Edit { name } => cmd::edit(&specs, &name)?,
        Commands::Rm {
            to_delete,
            skip_prompt,
        } => cmd::rm(&specs, to_delete, skip_prompt)?,
    }

    Ok(())
}
