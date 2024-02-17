mod cli;
mod config;
mod logging;

use clap::Parser as ClapParser;
use confique::{toml::FormatOptions, Config as DeriveConfig};
use miette::{bail, IntoDiagnostic};
use tracing::*;

use crate::cli::Cli;
use crate::cli::Cmd;
use crate::config::Config;

fn main() -> miette::Result<()> {
    logging::setup_logging();

    let cli = Cli::parse();
    debug!(?cli);

    let exe_path = std::env::current_exe().into_diagnostic()?;
    let config_path = exe_path.parent().unwrap().join("config.toml");
    debug!(?config_path);

    debug!("config exists: {}", config_path.exists());
    let _config = if cli.command != Cmd::GenerateConfig {
        info!("trying to read config from `{}`", config_path.display());
        if !config_path.exists() {
            info!("no existing config detected");
            info!("you can generate a default config via `generate-config` command");
            info!("the tool will now exit");
            return Ok(());
        }

        let config = Config::from_file(&config_path)
            .inspect_err(|e| {
                warn!("failed to load config from `{}`", config_path.display());
                warn!("default config values will be used");
                warn!(?e);
            })
            .unwrap_or_default();
        debug!(?config);
        config
    } else {
        Config::default()
    };

    match &cli.command {
        Cmd::GenerateConfig => {
            if !config_path.exists() {
                info!("generating config at `{}`", config_path.display());
                let template = confique::toml::template::<Config>(FormatOptions::default());
                std::fs::write(&config_path, template).into_diagnostic()?;
            } else {
                error!("config.toml already exists");
                bail!("config.toml already exists!");
            }
        }
        Cmd::GenerateTemplate { .. } => {
            todo!()
        }
        Cmd::GenerateReport { .. } => {
            todo!()
        }
    }

    Ok(())
}
