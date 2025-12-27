use crate::error::Error;
use crate::exec::CommandOsExt;
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

    /// Runs the executable command and captures stdout as string
    pub fn exec(self) -> Result<String, Error> {
        Command::shell_stdout0(&self.command)
    }
}
