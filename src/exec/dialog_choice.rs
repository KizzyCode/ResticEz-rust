use crate::error::Error;
use crate::exec::CommandExt;
use std::process::Command;

/// A choice dialogue
#[derive(Debug)]
pub struct DialogChoice {
    /// The underlying generic executable
    command: Command,
}
impl DialogChoice {
    /// Creates a new choice dialog command
    pub fn new<M, O, OSA, OSB>(message: M, options: O) -> Result<Self, Error>
    where
        M: ToString,
        O: IntoIterator<Item = (OSA, OSB)>,
        OSA: ToString,
        OSB: ToString,
    {
        // Initialize the dialog command
        let mut command = Command::new("dialog");
        command.arg("--clear").arg("--stdout").arg("--menu").arg(message.to_string()).arg("0").arg("0").arg("0");
        for (tag, desc) in options {
            // Append tags and descriptions
            command.arg(tag.to_string());
            command.arg(desc.to_string());
        }

        // Create the executable
        Ok(Self { command })
    }

    /// Shows the menu dialog and captures the input as string
    pub fn exec(mut self) -> Result<String, Error> {
        self.command.stdout0()
    }
}
