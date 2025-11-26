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
    /// List templates in the templates directory
    Ls {
        /// List the default arguments for each template
        #[arg(short, long)]
        list_vars: bool,
    },

    /// Create a new, blank template. The new template will be opened in your $EDITOR, unless
    /// --no-edit is specified.
    New {
        /// The name of your new template
        name: String,

        /// Don't open it in your $EDITOR after creation
        #[arg(long = "no-edit", default_value_t = true, action = clap::ArgAction::SetFalse)]
        edit: bool,
    },

    /// Generate a template, with options if specified in your template
    Gen {
        /// The template's name
        name: String,

        /// Options as key-value pairs (can be specified multiple times)
        #[arg(short = 'o', value_names = ["KEY", "VALUE"], num_args = 2)]
        options: Vec<String>,
    },

    /// Open a template in your editor of choice
    Edit {
        /// The template's name
        name: String,
    },
}
