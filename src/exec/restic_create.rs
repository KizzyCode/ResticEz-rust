use crate::config::Config;
use crate::error::Error;
use crate::exec::{CommandExt, Exec};
use std::process::Command;

/// Creates a new backup/snapshot
#[derive(Debug)]
pub struct ResticCreate {
    /// The underlying generic executable
    command: Command,
}
impl ResticCreate {
    /// Creates a command to create a new backup/snapshot
    pub fn new<S>(config: &Config, tags: &[S]) -> Result<Self, Error>
    where
        S: AsRef<str>,
    {
        // Group backup args
        let tags = tags.iter().flat_map(|t| ["--tag", t.as_ref()]);
        let flags = config.restic.flags.backup.iter().map(|f| f.as_str());
        let dirs = config.restic.dirs.iter().map(|d| d.as_str());

        // Assemble restic backup command
        let mut command = Command::new("restic");
        command.arg("backup").args(tags).args(flags).args(dirs);
        command.envs(config.to_env()?);

        Ok(Self { command })
    }
}
impl Exec for ResticCreate {
    type Output = ();

    fn exec(mut self) -> Result<Self::Output, Error> {
        self.command.exit0()
    }
}
