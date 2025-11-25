use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// List templates in the templates directory
    Ls {},

    /// Create a new, blank template
    New { name: String },

    /// Generate a template, with options if specified in your template
    Gen {
        /// Template name
        name: String,

        /// Options as key-value pairs (can be specified multiple times)
        #[arg(short = 'o', value_names = ["KEY", "VALUE"], num_args = 2)]
        options: Vec<String>,
    },

    /// Open a template in your editor of choice
    Edit {},
}
