use crate::{ config::Config, error::Result, exec::GenericExecutable };


/// Removes a stale lock on a restic repository
#[derive(Debug)]
pub struct ResticBreakLock {
    /// The underlying generic executable
    exec: GenericExecutable
}
impl ResticBreakLock {
    /// Creates a command to remove a stale lock on a restic repository
    pub fn new(config: &Config) -> Result<Self> {
        let restic = GenericExecutable::find("restic")?;
        let mut exec = GenericExecutable::new(restic, ["unlock"]);
        exec.set_envs(config.to_env()?);

        Ok(Self { exec })
    }
    
    /// Removes a stale lock on the restic repository
    pub fn exec(self) -> Result {
        self.exec.exec()
    }
}
