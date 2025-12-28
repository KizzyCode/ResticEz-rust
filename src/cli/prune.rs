use crate::cli::Command;
use crate::config::Config;
use crate::error::Error;
use crate::exec::Exec;
use crate::exec::dialog_info::DialogInfo;
use crate::exec::restic_prune::ResticPrune;

/// Prunes all unused chunks from the repository
#[derive(Debug)]
pub struct Prune {
    /// The config
    config: Config,
}
impl Command for Prune {
    fn new(config: Config) -> Self {
        Self { config }
    }

    fn exec(self) -> Result<(), Error> {
        DialogInfo::new("Pruning unused chunks...")?.exec()?;
        ResticPrune::new(&self.config)?.exec()
    }
}
