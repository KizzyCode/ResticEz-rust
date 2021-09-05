use crate::{ config::Config, error::Result, exec::GenericExecutable };


/// Checks the repository consistency
#[derive(Debug)]
pub struct ResticCheck {
    /// The underlying generic executable
    exec: GenericExecutable
}
impl ResticCheck {
    /// Creates a command to check the repository consistency
    pub fn new(config: &Config) -> Result<Self> {
        let restic = GenericExecutable::find("restic")?;
        let mut exec = GenericExecutable::new(restic, ["check", "--read-data"]);
        exec.set_envs(config.to_env()?);

        Ok(Self { exec })
    }
    
    /// Checks the repository consistency
    pub fn exec(self) -> Result {
        self.exec.exec()
    }
}
