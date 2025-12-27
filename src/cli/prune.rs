use crate::config::Config;
use crate::error::Result;
use crate::exec::dialog_info::DialogInfo;
use crate::exec::restic_prune::ResticPrune;

/// Prunes all unused chunks from the repository
pub struct Prune {
    /// The config
    config: Config,
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
