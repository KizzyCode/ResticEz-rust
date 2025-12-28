use crate::cli::Command;
use crate::config::Config;
use crate::error::Error;
use crate::exec::Exec;
use crate::exec::dialog_confirm::DialogConfirm;
use crate::exec::dialog_info::DialogInfo;
use crate::exec::restic_check::ResticCheck;

/// Checks the restic repository
#[derive(Debug)]
pub struct Check {
    /// The config
    config: Config,
}
impl Command for Check {
    fn new(config: Config) -> Self {
        Self { config }
    }

    fn exec(self) -> Result<(), Error> {
        let message = "Do you really want to check the repository (this may take a very long time)?";
        DialogConfirm::new(message, "Check repository", "Cancel")?.exec()?;
        DialogInfo::new("Checking repository (this may take a very long time)...")?.exec()?;
        ResticCheck::new(&self.config)?.exec()
    }
}
