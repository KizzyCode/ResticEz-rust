use crate::config::Config;
use crate::error::Error;
use crate::exec::{CommandExt, CommandOsExt};
use std::process::Command;

/// Opens a `shell` session with the given environment set
#[derive(Debug)]
pub struct ShellSession {
    /// The underlying raw command
    command: Command,
}
impl ShellSession {
    /// Creates a new `shell` session with `config` as environment
    pub fn new(config: &Config) -> Result<Self, Error> {
        // Create the shell command
        let shell = Command::shell()?;
        let mut command = Command::new(shell);
        command.envs(config.to_env()?);

        Ok(Self { command })
    }

    /// Creates the shell session
    pub fn exec(mut self) -> Result<(), Error> {
        eprintln!();
        eprintln!("======= Spawning new subshell: =======");
        self.command.exit0()
    }
}
