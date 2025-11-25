mod cli;
mod ls;
mod new;
mod template;

use clap::Parser;
use std::{fs, process::exit};

use anyhow::Result;
use directories::ProjectDirs;

use crate::{
    cli::{Cli, Commands},
    ls::ls,
    new::new,
};

fn main() -> Result<()> {
    let cli = Cli::parse();

    let template_directory = match ProjectDirs::from("org", "mcbunkus", "tmpl") {
        Some(proj_dirs) => proj_dirs.data_dir().to_path_buf(),
        None => {
            eprintln!("Unable to find base directories");
            exit(1);
        }
    };

    fs::create_dir_all(&template_directory)?;

    match cli.command {
        Commands::Ls {} => ls(template_directory)?,
        Commands::New { name } => new(template_directory, name)?,
        Commands::Gen { name, options } => todo!(),
        Commands::Edit {} => todo!(),
    }

    Ok(())
}
