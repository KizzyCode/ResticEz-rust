use crate::error::Error;
use crate::exec::{CommandExt, Exec};
use std::process::Command;

/// A credentials dialogue
#[derive(Debug)]
pub struct DialogCreds {
    /// The underlying generic executable
    command: Command,
}
impl DialogCreds {
    /// Creates a new credentials dialog command
    pub fn new<T>(message: T) -> Result<Self, Error>
    where
        T: AsRef<str>,
    {
        let mut command = Command::new("dialog");
        (command.arg("--clear"))
            .arg("--stdout")
            .arg("--insecure")
            .arg("--passwordbox")
            .arg(message.as_ref())
            .arg("8")
            .arg("80");

        Ok(Self { command })
    }
}
impl Exec for DialogCreds {
    type Output = String;

    fn exec(mut self) -> Result<Self::Output, Error> {
        self.command.stdout0()
    }
}
