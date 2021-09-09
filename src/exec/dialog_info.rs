use crate::error::Result;
use ezexec::ExecBuilder;


/// A information dialog
#[derive(Debug)]
pub struct DialogInfo {
    /// The underlying generic executable
    exec: ExecBuilder
}
impl DialogInfo {
    /// Creates a new information dialog command
    pub fn new<T>(message: T) -> Result<Self> where T: AsRef<str> {
        let exec = ExecBuilder::with_name("dialog", ["--stdout", "--infobox", message.as_ref(), "0", "0"])?;
        Ok(Self { exec })
    }
    
    /// Shows the information dialog
    pub fn exec(self) -> Result {
        Ok(self.exec.spawn_transparent()?.wait()?)
    }
}
