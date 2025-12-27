use crate::error::Result;
use ezexec::ExecBuilder;

/// A confirmation dialog
#[derive(Debug)]
pub struct DialogConfirm {
    /// The underlying generic executable
    exec: ExecBuilder,
}
impl DialogConfirm {
    /// Creates a new confirmation dialog command
    pub fn new<T, Y, N>(message: T, yes: Y, no: N) -> Result<Self>
    where
        T: AsRef<str>,
        Y: AsRef<str>,
        N: AsRef<str>,
    {
        let exec = ExecBuilder::with_name(
            "dialog",
            [
                "--clear",
                "--defaultno",
                "--yes-label",
                yes.as_ref(),
                "--no-label",
                no.as_ref(),
                "--yesno",
                message.as_ref(),
                "0",
                "0",
            ],
        )?;
        Ok(Self { exec })
    }

    /// Shows the confirmation dialog
    pub fn exec(self) -> Result {
        Ok(self.exec.spawn_transparent()?.wait()?)
    }
}
