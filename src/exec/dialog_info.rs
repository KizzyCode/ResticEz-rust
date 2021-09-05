use crate::{ error::Result, exec::GenericExecutable };


/// A information dialog
#[derive(Debug)]
pub struct DialogInfo {
    /// The underlying generic executable
    exec: GenericExecutable
}
impl DialogInfo {
    /// Creates a new information dialog command
    pub fn new<T>(message: T) -> Result<Self> where T: AsRef<str> {
        let dialog = GenericExecutable::find("dialog")?;
        let exec = GenericExecutable::new(dialog, ["--stdout", "--infobox", message.as_ref(), "0", "0"]);
        Ok(Self { exec })
    }
    
    /// Shows the information dialog
    pub fn exec(self) -> Result {
        self.exec.exec()
    }
}
