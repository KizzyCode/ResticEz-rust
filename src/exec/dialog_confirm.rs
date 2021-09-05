use crate::{ error::Result, exec::GenericExecutable };


/// A confirmation dialog
#[derive(Debug)]
pub struct DialogConfirm {
    /// The underlying generic executable
    exec: GenericExecutable
}
impl DialogConfirm {
    /// Creates a new confirmation dialog command
    pub fn new<T, Y, N>(message: T, yes: Y, no: N) -> Result<Self> where T: AsRef<str>, Y: AsRef<str>, N: AsRef<str> {
        let dialog = GenericExecutable::find("dialog")?;
        let exec = GenericExecutable::new(dialog, [
            "--clear", "--defaultno", "--yes-label", yes.as_ref(), "--no-label", no.as_ref(),
            "--yesno", message.as_ref(), "0", "0"
        ]);
        Ok(Self { exec })
    }
    
    /// Shows the confirmation dialog
    pub fn exec(self) -> Result {
        self.exec.exec()
    }
}
