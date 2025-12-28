use crate::config::Config;
use crate::error::Error;
use crate::exec::{CommandExt, Exec};
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
}
impl Exec for ResticCheck {
    type Output = ();

    fn exec(mut self) -> Result<Self::Output, Error> {
        self.command.exit0()
    }
}
