use crate::error::Error;
use crate::exec::CommandExt;
use std::process::Command;

/// A information dialog
#[derive(Debug)]
pub struct DialogInfo {
    /// The underlying generic executable
    command: Command,
}
impl DialogInfo {
    /// Creates a new information dialog command
    pub fn new<T>(message: T) -> Result<Self, Error>
    where
        T: AsRef<str>,
    {
        let mut command = Command::new("dialog");
        command.arg("--stdout").arg("--infobox").arg(message.as_ref()).arg("0").arg("0");

        Ok(Self { command })
    }

    /// Shows the information dialog
    pub fn exec(mut self) -> Result<(), Error> {
        self.command.exit0()
    }
}
