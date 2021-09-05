use crate::{
    config::Config, error::Result,
    exec::{ dialog_confirm::DialogConfirm, dialog_info::DialogInfo, restic_check::ResticCheck }
};


/// Checks the restic repository
pub struct Check {
    /// The config
    config: Config
}
impl Check {
    /// Creates a command to check the restic repository
    pub const fn new(config: Config) -> Self {
        Self { config }
    }

    /// Executes the command
    pub fn exec(self) -> Result {
        let message = "Do you really want to check the repository (this may take a very long time)?";
        DialogConfirm::new(message, "Check repository", "Cancel")?.exec()?;
        DialogInfo::new("Checking repository (this may take a very long time)...")?.exec()?;
        ResticCheck::new(&self.config)?.exec()
    }
}