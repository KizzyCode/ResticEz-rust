use crate::error::Error;
use crate::exec::Exec;
use crate::exec::dialog_creds::DialogCreds;
use crate::exec::shell_command::ShellCommand;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;

/// A dynamically sourced argument
#[derive(Debug, Serialize, Deserialize)]
pub struct DynamicArgument {
    /// The argument value
    pub value: Option<String>,
    /// A shell command to obtain the argument value
    pub command: Option<String>,
    /// A message to use when asking for user input to get the value
    pub message: Option<String>,
    /// The cached value
    #[serde(skip)]
    cached: RefCell<Option<String>>,
}
impl DynamicArgument {
    /// Evaluates the argument and gets the value
    pub fn eval(&self) -> Result<String, Error> {
        // Populate the cache if not done yet
        if self.cached.borrow().is_none() {
            let value = self._eval()?;
            *self.cached.borrow_mut() = Some(value);
        }

        // Get the cached value
        let cached = self.cached.borrow().clone().expect("Failed to get cached value?!");
        Ok(cached)
    }
    /// Evaluates the argument and gets the value
    fn _eval(&self) -> Result<String, Error> {
        if let Some(value) = self.value.to_owned() {
            return Ok(value);
        }
        if let Some(command) = self.command.as_ref() {
            let shell = ShellCommand::new(command)?;
            let shell_out = shell.exec()?;
            return Ok(shell_out.trim().to_string());
        }
        if let Some(message) = self.message.as_ref() {
            let dialog = DialogCreds::new(message)?;
            return Ok(dialog.exec()?);
        }
        Err(einval!("Cannot evaluate unconfigured dynamic argument"))
    }
}
