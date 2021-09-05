#[macro_use] mod error;
mod cli;
mod config;
mod exec;
mod fs;

use crate::{ cli::CliCommand, config::Config, error::Result };
use std::{ env, process };


/// Gathers the config
fn gather_config() -> Result<String> {
    // Get the config from an environment variable
    if let Ok(config) = env::var("RESTIC_EZ_CONFIG_TOML") {
        return Ok(config);
    }

    // Get the config from a file
    if let Ok(path) = env::var("RESTIC_EZ_CONFIG") {
        return fs::read_string(path);
    }
    if let Ok(config) = fs::read_string("restic-ez") {
        return Ok(config);
    }
    if let Ok(config) = fs::read_string("restic-ez.conf") {
        return Ok(config);
    }
    if let Ok(config) = fs::read_string("restic-ez.toml") {
        return Ok(config);
    }
    if let Ok(config) = fs::read_string(".restic-ez") {
        return Ok(config);
    }
    if let Ok(config) = fs::read_string(".restic-ez.conf") {
        return Ok(config);
    }
    if let Ok(config) = fs::read_string(".restic-ez.toml") {
        return Ok(config);
    }

    // No config available
    Err(eio!("Failed to locate a valid config"))
}


/// Main entry point
fn main() {
    /// The real main function
    fn _main() -> Result {
        let config_string = gather_config()?;
        let config = Config::from_str(config_string)?;
        CliCommand::new(config).exec()
    }

    // Execute the main block or pretty-print the error
    if let Err(err) = _main() {
        eprintln!();
        eprintln!("Error: {}", err);
        process::exit(1);
    }
}
