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
    /// Given a list of PR IDs, fetch their information, and generate a form with some of the
    /// information filled out. This information should be provided through the config file. You can
    /// specify the path for the template report.
    HydrateForm {
        /// Desired output path for the partially hydrated form.
        out_form_path: PathBuf,
    },
    /// Format a PR triage report in markdown using information from a fully filled out form.
    FormatReport {
        in_form_path: PathBuf,
        /// Desired output path for the manually completed form.
        out_report_path: PathBuf,
    },
}
