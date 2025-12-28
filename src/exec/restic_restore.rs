use crate::config::Config;
use crate::error::Error;
use crate::exec::{CommandExt, Exec};
use std::process::Command;

/// Restores the latest snapshot
#[derive(Debug)]
pub struct ResticRestore {
    /// The underlying generic executable
    command: Command,
}
impl ResticRestore {
    /// Creates a command to restore the latest snapshot with the given tags
    pub fn new<S>(config: &Config, tags: &[S]) -> Result<Self, Error>
    where
        S: AsRef<str>,
    {
        // Group restore args
        let tags = tags.iter().flat_map(|t| ["--tag", t.as_ref()]);
        let dirs = config.restic.dirs.iter().flat_map(|d| ["--path", d.as_str()]);

        // Assemble restic restore command
        let mut command = Command::new("restic");
        command.arg("restore").arg("latest").arg("--target").arg("/").args(tags).args(dirs);
        command.envs(config.to_env()?);

        Ok(Self { command })
    }
}
impl Exec for ResticRestore {
    type Output = ();

    fn exec(mut self) -> Result<Self::Output, Error> {
        self.command.exit0()
    }
}
