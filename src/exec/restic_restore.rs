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
    pub fn new<S>(config: &Config, tags: &[S]) -> Result<Self> where S: AsRef<str> {        
        // Create the executable flags
        let args = vec!["restore", "latest", "--target", "/"].into_iter()
            .chain(tags.iter().flat_map(|t| ["--tag", t.as_ref()]))
            .chain(config.restic.dirs.iter().flat_map(|d| ["--path", d.as_str()]));
        
        // Create the executable
        let mut exec = ExecBuilder::with_name("restic", args)?;
        exec.set_envs(config.to_env()?);
        Ok(Self { exec })
    }
    
    /// Creates the backup/snapshot
    pub fn exec(self) -> Result {
        Ok(self.exec.spawn_transparent()?.wait()?)
    }
}
