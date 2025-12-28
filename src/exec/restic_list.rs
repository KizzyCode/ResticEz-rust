use crate::config::Config;
use crate::error::Error;
use crate::exec::{CommandExt, Exec};
use serde::{Deserialize, Serialize};
use std::process::Command;

/// The info about a restic archive
#[derive(Debug, Serialize, Deserialize)]
pub struct ResticListArchive {
    /// The time stamp
    pub time: String,
    /// The paths included in the archive
    pub paths: Vec<String>,
    /// The name of the host who creates the archive
    pub hostname: String,
    /// The tags associated with the archive
    #[serde(default)]
    pub tags: Vec<String>,
    /// The ID of the archive
    pub id: String,
    /// The short ID of the archive
    pub short_id: String,
}
/// The restic list output
pub type ResticListOutput = Vec<ResticListArchive>;

/// A information dialog
#[derive(Debug)]
pub struct ResticList {
    /// The underlying generic executable
    command: Command,
}
impl ResticList {
    /// Creates a command to list the repository archives
    pub fn new(config: &Config) -> Result<Self, Error> {
        let mut command = Command::new("restic");
        command.arg("snapshots").arg("--json");
        command.envs(config.to_env()?);

        Ok(Self { command })
    }
}
impl Exec for ResticList {
    type Output = ResticListOutput;

    fn exec(mut self) -> Result<Self::Output, Error> {
        let output_json: String = self.command.stdout0()?;
        serde_json::from_str(&output_json).map_err(|e| einval!("Unexpected JSON output from `restic` command: {e}"))
    }
}
