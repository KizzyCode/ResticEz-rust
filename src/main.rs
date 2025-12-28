#![doc = include_str!("../README.md")]
// Clippy lints
#![warn(clippy::large_stack_arrays)]
#![warn(clippy::arithmetic_side_effects)]
#![warn(clippy::expect_used)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::indexing_slicing)]
#![warn(clippy::todo)]
#![warn(clippy::unimplemented)]
#![warn(clippy::missing_panics_doc)]
#![warn(clippy::allow_attributes_without_reason)]
#![warn(clippy::cognitive_complexity)]
#![forbid(unsafe_code)]

#[macro_use]
mod error;
mod cli;
mod config;
mod exec;
mod fs;

use crate::cli::CliCommand;
use crate::config::Config;
use crate::error::Error;
use std::iter::Peekable;
use std::{env, process};

/// Locates the configuration file/data
fn locate_config<Argv>(argv: &mut Peekable<Argv>) -> Result<String, Error>
where
    Argv: Iterator<Item = String>,
{
    // See if the first argument references a config file
    if let Some(path) = argv.next_if(|path| path.ends_with(".toml") || path.ends_with(".conf")) {
        return fs::read_string(path);
    }

    // Get the config from an environment variable
    if let Ok(config) = env::var("RESTIC_EZ_CONFIG_TOML") {
        return Ok(config);
    }
    if let Ok(path) = env::var("RESTIC_EZ_CONFIG") {
        return fs::read_string(path);
    }

    // Get the config from a default file
    if let Ok(config) = fs::read_string("restic-ez.conf") {
        return Ok(config);
    }
    if let Ok(config) = fs::read_string("restic-ez.toml") {
        return Ok(config);
    }
    if let Ok(config) = fs::read_string(".restic-ez.conf") {
        return Ok(config);
    }
    if let Ok(config) = fs::read_string(".restic-ez.toml") {
        return Ok(config);
    }

    // No config available
    Err(eio!("Failed to locate a valid configuration file"))
}

/// Main entry point
fn main() {
    /// The real main function
    fn _main() -> Result<(), Error> {
        // Get the given arguments, and consume arg0 which is the binary name
        let mut argv = env::args().peekable();
        let _ = argv.next();

        // Load config and invoke command
        let config_string = locate_config(&mut argv)?;
        let config = Config::from_str(config_string)?;
        CliCommand::new(config, &mut argv).exec()
    }

    // Execute the main block or pretty-print the error
    if let Err(err) = _main() {
        eprintln!();
        eprintln!("Error: {err}");
        process::exit(1);
    }
}
