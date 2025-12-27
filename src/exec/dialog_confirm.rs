use crate::error::Error;
use crate::exec::CommandExt;
use std::process::Command;

/// A confirmation dialog
#[derive(Debug)]
pub struct DialogConfirm {
    /// The underlying generic executable
    command: Command,
}
impl DialogConfirm {
    /// Creates a new confirmation dialog command
    pub fn new<T, Y, N>(message: T, yes: Y, no: N) -> Result<Self, Error>
    where
        T: AsRef<str>,
        Y: AsRef<str>,
        N: AsRef<str>,
    {
        let mut command = Command::new("dialog");
        (command.arg("--clear"))
            .arg("--defaultno")
            .arg("--yes-label")
            .arg(yes.as_ref())
            .arg("--no-label")
            .arg(no.as_ref())
            .arg("--yesno")
            .arg(message.as_ref())
            .arg("0")
            .arg("0");

        Ok(Self { command })
    }

    /// Shows the confirmation dialog
    pub fn exec(mut self) -> Result<(), Error> {
        self.command.exit0()
    }
}
