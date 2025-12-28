use crate::error::Error;
use crate::exec::{CommandExt, Exec};
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
}
impl Exec for DialogInfo {
    type Output = ();

    fn exec(mut self) -> Result<Self::Output, Error> {
        self.command.exit0()
    }
}
