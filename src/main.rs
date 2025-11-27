use std::fs;

use clap::Parser;
use directories::ProjectDirs;
use tmpl::Cli;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    // ensure the default spec directory exists before proceeding.
    // TODO: make the spec directory location configurable?
    let spec_dir = ProjectDirs::from("org", "mcbunkus", "tmpl")
        .ok_or_else(|| anyhow::anyhow!("Unable to find base directories"))?
        .data_dir()
        .to_path_buf();

    fs::create_dir_all(&spec_dir)?;

    tmpl::run(cli, &spec_dir)
}
