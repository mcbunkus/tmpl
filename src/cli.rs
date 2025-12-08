use std::{ffi::OsString, path::PathBuf};

use clap::{Parser, Subcommand};

/// tmpl is a barebones command line tool for generating multiple templated files from a single
/// TOML spec.
#[derive(Parser)]
#[command(version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Ls(LsArgs),
    New(NewArgs),
    Gen(GenArgs),
    Edit(EditArgs),
    Rm(RmArgs),
    Cp(CpArgs),
}

/// List specs in the specs directory
#[derive(Debug, clap::Args)]
pub struct LsArgs {
    /// List the default arguments for each spec
    #[arg(short, long)]
    pub list_vars: bool,
}

/// Create a new spec with some example content. The new spec will be opened in your $EDITOR, unless
/// --no-edit is specified
#[derive(Debug, clap::Args)]
pub struct NewArgs {
    /// The name of your new spec
    pub name: OsString,

    /// Don't open it in your $EDITOR after creation
    #[arg(long = "no-edit", default_value_t = true, action = clap::ArgAction::SetFalse)]
    pub edit: bool,
}

/// Generate templates from a spec, with options if specified in your spec file
#[derive(Debug, clap::Args)]
pub struct GenArgs {
    /// The spec's name. This is mutually exclusive with --file, and will result in an error if
    /// both are used
    pub name: Option<OsString>,

    /// Options as key-value pairs (can be specified multiple times)
    #[arg(short = 'o', value_names = ["KEY", "VALUE"], num_args = 2)]
    pub options: Vec<String>,

    /// The directory to generate the spec in.
    #[arg(short = 'c')]
    pub workdir: Option<PathBuf>,

    /// Point to a spec file, instead of using a spec in the spec directory. Mutually exclusive
    /// with NAME.
    #[arg(short = 'f', long = "file")]
    pub spec_file: Option<PathBuf>,
}

/// Open a spec in your editor of choice
#[derive(Debug, clap::Args)]
pub struct EditArgs {
    /// The spec's name
    pub name: OsString,
}

/// Delete one or more specs
#[derive(Debug, clap::Args)]
pub struct RmArgs {
    pub to_delete: Vec<OsString>,

    /// Confirm yes for all given specs
    #[arg(short = 'y', long = "yes", default_value_t = false, action = clap::ArgAction::SetTrue)]
    pub skip_prompt: bool,
}

/// Copy a spec
#[derive(Debug, clap::Args)]
pub struct CpArgs {
    /// The spec you want to copy
    pub source: OsString,

    /// The name of the new spec
    pub dest: OsString,

    /// Skip are you sure prompt
    #[arg(short = 'y', long = "yes", default_value_t = false, action = clap::ArgAction::SetTrue)]
    pub skip_prompt: bool,
}
