mod cli;
mod cmd;
mod template;

use clap::Parser;
use std::fs;

use anyhow::Result;
use directories::ProjectDirs;

use crate::{
    cli::{Cli, Commands},
    cmd::edit,
};

fn main() -> Result<()> {
    let cli = Cli::parse();

    let template_directory = ProjectDirs::from("org", "mcbunkus", "tmpl")
        .ok_or_else(|| anyhow::anyhow!("Unable to find base directories"))?
        .data_dir()
        .to_path_buf();

    fs::create_dir_all(&template_directory)?;

    // TODO: make this optional
    let editor = std::env::var_os("EDITOR").ok_or_else(|| anyhow::anyhow!("EDITOR environment variable is not set. Please set this environment variable to the path of your preferred text editor."))?;

    match cli.command {
        Commands::Ls { list_vars } => cmd::list(template_directory, list_vars)?,
        Commands::New { name, edit } => cmd::new(template_directory, name, edit, editor)?,
        Commands::Gen { name, options } => cmd::generate(template_directory, name, options)?,
        Commands::Edit { name } => edit(template_directory, name, editor)?,
    }

    Ok(())
}
