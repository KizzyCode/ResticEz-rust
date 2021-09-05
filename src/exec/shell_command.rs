use crate::{ error::Result, exec::GenericExecutable };


/// Executes a shell command
#[derive(Debug)]
pub struct ShellCommand {
    /// The underlying raw command
    exec: GenericExecutable
}
impl ShellCommand {
    /// Creates a new shell command
    pub fn new<T>(command: T) -> Result<Self> where T: ToString {
        let sh = GenericExecutable::find("sh")?;
        let exec = GenericExecutable::new(sh, ["-c".to_string(), command.to_string()]);
        Ok(Self { exec })
    }
    
    /// Runs the executable command and captures stdout as string
    pub fn exec_stringout(self) -> Result<String> {
        self.exec.exec_stringout()
    }
}
