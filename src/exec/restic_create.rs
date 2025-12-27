use crate::config::Config;
use crate::error::Result;
use ezexec::ExecBuilder;

/// Creates a new backup/snapshot
#[derive(Debug)]
pub struct ResticCreate {
    /// The underlying generic executable
    exec: ExecBuilder,
}
impl ResticCreate {
    /// Creates a command to create a new backup/snapshot
    pub fn new<S>(config: &Config, tags: &[S]) -> Result<Self>
    where
        S: AsRef<str>,
    {
        // Create the executable flags
        let args = vec!["backup"]
            .into_iter()
            .chain(tags.iter().flat_map(|t| ["--tag", t.as_ref()]))
            .chain(config.restic.flags.backup.iter().map(|f| f.as_str()))
            .chain(config.restic.dirs.iter().map(|d| d.as_str()));

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
