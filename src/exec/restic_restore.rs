use crate::{ config::Config, error::Result };
use ezexec::ExecBuilder;


/// Restores the latest snapshot
#[derive(Debug)]
pub struct ResticRestore {
    /// The underlying generic executable
    exec: ExecBuilder
}
impl ResticRestore {
    /// Creates a command to restore the latest snapshot with the given tags
    pub fn new<S, I>(config: &Config, tags: I) -> Result<Self> where I: IntoIterator<Item = S>, S: ToString {        
        // Create the executable
        let tags = tags.into_iter().map(|a| a.to_string()).collect::<Vec<_>>().join(",");
        let mut exec = ExecBuilder::with_name("restic", [
            "restore", "latest", "--target", "/",
            "--tag", &tags, "--path", &config.restic.dir
        ])?;
        exec.set_envs(config.to_env()?);

        Ok(Self { exec })
    }
    
    /// Creates the backup/snapshot
    pub fn exec(self) -> Result {
        Ok(self.exec.spawn_transparent()?.wait()?)
    }
}
