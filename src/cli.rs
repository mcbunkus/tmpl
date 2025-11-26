use std::ffi::OsString;

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
    /// List specs in the specs directory
    Ls {
        /// List the default arguments for each spec
        #[arg(short, long)]
        list_vars: bool,
    },

    /// Create a new spec with some example content. The new spec will be opened in your $EDITOR, unless
    /// --no-edit is specified
    New {
        /// The name of your new spec
        name: OsString,

        /// Don't open it in your $EDITOR after creation
        #[arg(long = "no-edit", default_value_t = true, action = clap::ArgAction::SetFalse)]
        edit: bool,
    },

    /// Generate templates from a spec, with options if specified in your spec file
    Gen {
        /// The spec's name
        name: OsString,

        /// Options as key-value pairs (can be specified multiple times)
        #[arg(short = 'o', value_names = ["KEY", "VALUE"], num_args = 2)]
        options: Vec<String>,
    },

    /// Open a spec in your editor of choice
    Edit {
        /// The spec's name
        name: OsString,
    },

    /// Delete one or more specs
    Rm {
        to_delete: Vec<OsString>,

        /// Confirm yes for all given specs
        #[arg(short = 'y', long = "yes", default_value_t = false, action = clap::ArgAction::SetTrue)]
        skip_prompt: bool,
    },
}
