use crate::config::Config;
use crate::error::Error;
use crate::exec::CommandExt;
use std::process::Command;

/// Checks the repository consistency
#[derive(Debug)]
pub struct ResticCheck {
    /// The underlying generic executable
    command: Command,
}
impl ResticCheck {
    /// Creates a command to check the repository consistency
    pub fn new(config: &Config) -> Result<Self, Error> {
        let mut command = Command::new("restic");
        command.arg("check").arg("--read-data");
        command.envs(config.to_env()?);

        Ok(Self { command })
    }

    /// Checks the repository consistency
    pub fn exec(mut self) -> Result<(), Error> {
        self.command.exit0()
    }
}
