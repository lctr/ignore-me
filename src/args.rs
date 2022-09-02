pub use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
pub struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,

    #[clap(short, long, value_parser, value_name = "BOOL")]
    debug: bool,
}

impl Cli {
    pub fn get() -> Self {
        Self::parse()
    }

    pub fn debug(&self) -> bool {
        self.debug
    }

    pub fn command(&self) -> Option<&Commands> {
        self.command.as_ref()
    }
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Adds to the existing `.gitignore` file the results of the
    /// search containing the provided search terms and/or names.
    Add {
        #[clap(short, long, value_parser)]
        terms: Vec<String>,

        #[clap(value_parser)]
        names: Vec<String>,
    },
    /// Use provided search terms to determine which `.gitignore` file(s)
    /// will be sourced.
    Terms {
        #[clap(value_parser)]
        terms: Vec<String>,
    },
    /// Use provided names to determine which `.gitignore` file(s)
    /// will be sourced.
    For {
        #[clap(value_parser)]
        names: Vec<String>,
    },
}
