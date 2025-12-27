use crate::config::Config;
use crate::error::Error;
use crate::exec::CommandExt;
use std::process::Command;

/// Removes a stale lock on a restic repository
#[derive(Debug)]
pub struct ResticBreakLock {
    /// The underlying generic executable
    command: Command,
}
impl ResticBreakLock {
    /// Creates a command to remove a stale lock on a restic repository
    pub fn new(config: &Config) -> Result<Self, Error> {
        let mut command = Command::new("restic");
        command.arg("unlock");
        command.envs(config.to_env()?);

        Ok(Self { command })
    }

    /// Removes a stale lock on the restic repository
    pub fn exec(mut self) -> Result<(), Error> {
        self.command.exit0()
    }
}
