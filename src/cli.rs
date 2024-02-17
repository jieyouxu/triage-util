use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Cmd,
}

#[derive(Debug, PartialEq, Subcommand)]
pub enum Cmd {
    /// Generate a default configuration file under the same directory as the executable.
    GenerateConfig,
    /// Given a list of PR IDs, fetch their information, and generate a template report with
    /// some of the information filled out. This information should be provided through the
    /// config file. You can specify the path for the template report.
    GenerateTemplate { path: PathBuf },
    /// Given a
    GenerateReport {
        /// Path to the filled report template.
        path: PathBuf,
    },
}
