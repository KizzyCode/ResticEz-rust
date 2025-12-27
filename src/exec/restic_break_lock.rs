use crate::config::Config;
use crate::error::Result;
use ezexec::ExecBuilder;

/// Removes a stale lock on a restic repository
#[derive(Debug)]
pub struct ResticBreakLock {
    /// The underlying generic executable
    exec: ExecBuilder,
}
impl ResticBreakLock {
    /// Creates a command to remove a stale lock on a restic repository
    pub fn new(config: &Config) -> Result<Self> {
        let mut exec = ExecBuilder::with_name("restic", ["unlock"])?;
        exec.set_envs(config.to_env()?);

        Ok(Self { exec })
    }

    /// Removes a stale lock on the restic repository
    pub fn exec(self) -> Result {
        Ok(self.exec.spawn_transparent()?.wait()?)
    }
}
