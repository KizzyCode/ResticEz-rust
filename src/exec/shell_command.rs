use crate::error::Result;
use ezexec::ExecBuilder;
use std::convert::TryInto;

/// Executes a shell command
#[derive(Debug)]
pub struct ShellCommand {
    /// The underlying raw command
    exec: ExecBuilder,
}
impl ShellCommand {
    /// Creates a new shell command
    pub fn new<T>(command: T) -> Result<Self>
    where
        T: AsRef<str>,
    {
        let exec = ExecBuilder::with_shell(command)?;
        Ok(Self { exec })
    }

    /// Runs the executable command and captures stdout as string
    pub fn exec_stringout(self) -> Result<String> {
        Ok(self.exec.spawn_captured()?.try_into()?)
    }
}
