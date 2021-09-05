use crate::{ config::Config, error::Result, exec::GenericExecutable };
use serde::{ Serialize, Deserialize };


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
    exec: GenericExecutable
}
impl ResticList {
    /// Creates a command to list the repository archives
    pub fn new(config: &Config) -> Result<Self> {
        let restic = GenericExecutable::find("restic")?;
        let mut exec = GenericExecutable::new(restic, ["snapshots", "--json"]);
        exec.set_envs(config.to_env()?);

        Ok(Self { exec })
    }
    
    /// Lists the repository archives
    pub fn exec(self) -> Result<ResticListOutput> {
        let output_json = self.exec.exec_stringout()?;
        serde_json::from_str(&output_json).map_err(|e| einval!("Unexpected JSON output from `restic` command ({})", e))
    }
}
