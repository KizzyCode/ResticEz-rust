use crate::config::Config;
use crate::error::Error;
use crate::exec::{CommandExt, Exec};
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
}
impl Exec for ResticPrune {
    type Output = ();

    fn exec(mut self) -> Result<Self::Output, Error> {
        self.command.exit0()
    }
}
