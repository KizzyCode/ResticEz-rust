use crate::{
    config::Config, error::Result,
    exec::{ dialog_info::DialogInfo, restic_prune::ResticPrune }
};


/// Prunes all unused chunks from the repository
pub struct Prune {
    /// The config
    config: Config
}
impl Prune {
    /// Creates a command to prune all unused chunks from the repository
    pub const fn new(config: Config) -> Self {
        Self { config }
    }

    /// Executes the command
    pub fn exec(self) -> Result {
        DialogInfo::new("Pruning unused chunks...")?.exec()?;
        ResticPrune::new(&self.config)?.exec()
    }
}
