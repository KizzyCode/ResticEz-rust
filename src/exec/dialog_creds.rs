use crate::{ error::Result, exec::GenericExecutable };


/// A credentials dialogue
#[derive(Debug)]
pub struct DialogCreds {
    /// The underlying generic executable
    exec: GenericExecutable
}
impl DialogCreds {
    /// Creates a new credentials dialog command
    pub fn new<T>(message: T) -> Result<Self> where T: AsRef<str> {
        let dialog = GenericExecutable::find("dialog")?;
        let exec = GenericExecutable::new(dialog, [
            "--clear", "--stdout", "--insecure",
            "--passwordbox", message.as_ref(), "8", "80"
        ]);
        Ok(Self { exec })
    }
    
    /// Shows the credentials dialog and captures the input as string
    pub fn exec(self) -> Result<String> {
        self.exec.exec_stringout()
    }
}
