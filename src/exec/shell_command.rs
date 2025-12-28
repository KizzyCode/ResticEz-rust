use crate::error::Error;
use crate::exec::{CommandOsExt, Exec};
use std::process::Command;

/// Executes a shell command
#[derive(Debug)]
pub struct ShellCommand {
    /// The underlying raw command
    command: String,
}
impl ShellCommand {
    /// Creates a new shell command
    pub fn new<T>(command: T) -> Result<Self, Error>
    where
        T: ToString,
    {
        let command = command.to_string();
        Ok(Self { command })
    }
}
impl Exec for ShellCommand {
    type Output = String;

    fn exec(self) -> Result<Self::Output, Error> {
        Command::shell_stdout0(&self.command)
    }
}
