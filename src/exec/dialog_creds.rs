use crate::error::Result;
use ezexec::ExecBuilder;
use std::convert::TryInto;


/// A credentials dialogue
#[derive(Debug)]
pub struct DialogCreds {
    /// The underlying generic executable
    exec: ExecBuilder
}
impl DialogCreds {
    /// Creates a new credentials dialog command
    pub fn new<T>(message: T) -> Result<Self> where T: AsRef<str> {
        let exec = ExecBuilder::with_name("dialog", [
            "--clear", "--stdout", "--insecure",
            "--passwordbox", message.as_ref(), "8", "80"
        ])?;
        Ok(Self { exec })
    }
    
    /// Shows the credentials dialog and captures the input as string
    pub fn exec(self) -> Result<String> {
        Ok(self.exec.spawn_captured()?.try_into()?)
    }
}
