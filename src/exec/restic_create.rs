use crate::{ config::Config, error::Result, exec::GenericExecutable };


/// Creates a new backup/snapshot
#[derive(Debug)]
pub struct ResticCreate {
    /// The underlying generic executable
    exec: GenericExecutable
}
impl ResticCreate {
    /// Creates a command to create a new backup/snapshot
    pub fn new<S, I>(config: &Config, tags: I) -> Result<Self> where I: IntoIterator<Item = S>, S: ToString {
        // Look for restic and concatenate the tags
        let restic = GenericExecutable::find("restic")?;
        let tags = tags.into_iter().map(|a| a.to_string()).collect::<Vec<_>>().join(",");
        
        // Create the executable
        let mut exec = GenericExecutable::new(restic, ["backup", "--tag", &tags, &config.restic.dir]);
        exec.set_envs(config.to_env()?);
        Ok(Self { exec })
    }
    
    /// Creates the backup/snapshot
    pub fn exec(self) -> Result {
        self.exec.exec()
    }
}
