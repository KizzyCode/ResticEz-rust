use crate::{ config::Config, error::Result, exec::GenericExecutable };


/// Prunes all unused chunks from the repository
#[derive(Debug)]
pub struct ResticPrune {
    /// The underlying generic executable
    exec: GenericExecutable
}
impl ResticPrune {
    /// Creates a command to prune all unused chunks from the repository
    pub fn new(config: &Config) -> Result<Self> {
        let restic = GenericExecutable::find("restic")?;
        let mut exec = GenericExecutable::new(restic, ["prune"]);
        exec.set_envs(config.to_env()?);

        Ok(Self { exec })
    }
    
    /// Prunes all unused chunks from the repository
    pub fn exec(self) -> Result {
        self.exec.exec()
    }
}
