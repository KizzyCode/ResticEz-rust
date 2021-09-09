use crate::{ config::Config, error::Result };
use ezexec::ExecBuilder;


/// Creates a new backup/snapshot
#[derive(Debug)]
pub struct ResticCreate {
    /// The underlying generic executable
    exec: ExecBuilder
}
impl ResticCreate {
    /// Creates a command to create a new backup/snapshot
    pub fn new<S, I>(config: &Config, tags: I) -> Result<Self> where I: IntoIterator<Item = S>, S: ToString {
        // Create the executable flags
        let tags = tags.into_iter().map(|a| a.to_string()).collect::<Vec<_>>().join(",");
        let args = vec!["backup", "--tag", &tags].into_iter()
            .chain(config.restic.flags.backup.iter().map(|f| f.as_str()))
            .chain(vec![config.restic.dir.as_str()]);

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
