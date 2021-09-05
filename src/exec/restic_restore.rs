use crate::{ config::Config, error::Result, exec::GenericExecutable };


/// Restores the latest snapshot
#[derive(Debug)]
pub struct ResticRestore {
    /// The underlying generic executable
    exec: GenericExecutable
}
impl ResticRestore {
    /// Creates a command to restore the latest snapshot with the given tags
    pub fn new<S, I>(config: &Config, tags: I) -> Result<Self> where I: IntoIterator<Item = S>, S: ToString {
        // Look for restic and concatenate the tags
        let restic = GenericExecutable::find("restic")?;
        let tags = tags.into_iter().map(|a| a.to_string()).collect::<Vec<_>>().join(",");
        
        // Create the executable
        let mut exec = GenericExecutable::new(restic, [
            "restore", "latest", "--target", "/",
            "--tag", &tags, "--path", &config.restic.dir
        ]);
        exec.set_envs(config.to_env()?);
        Ok(Self { exec })
    }
    
    /// Creates the backup/snapshot
    pub fn exec(self) -> Result {
        self.exec.exec()
    }
}
