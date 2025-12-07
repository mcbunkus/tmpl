use std::fs;

use anyhow::Context;
use clap::Parser;
use directories::ProjectDirs;
use tmpl::{
    cli::{self, Cli},
    cmd,
    io::IO,
    specs::Specs,
};

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    // ensure the default spec directory exists before proceeding.
    // TODO: make the spec directory location configurable?
    let spec_dir = ProjectDirs::from("org", "mcbunkus", "tmpl")
        .context("Unable to find base directories")?
        .data_dir()
        .to_path_buf();

    fs::create_dir_all(&spec_dir)?;

    let specs = Specs::new(&spec_dir)?;
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
