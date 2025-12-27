use crate::config::Config;
use crate::error::Error;
use crate::exec::CommandExt;
use std::process::Command;

/// Prunes all unused chunks from the repository
#[derive(Debug)]
pub struct ResticPrune {
    /// The underlying generic executable
    command: Command,
}
impl ResticPrune {
    /// Creates a command to prune all unused chunks from the repository
    pub fn new(config: &Config) -> Result<Self, Error> {
        let mut command = Command::new("restic");
        command.arg("prune");
        command.envs(config.to_env()?);

        Ok(Self { command })
    }

    /// Prunes all unused chunks from the repository
    pub fn exec(mut self) -> Result<(), Error> {
        self.command.exit0()
    }
}
