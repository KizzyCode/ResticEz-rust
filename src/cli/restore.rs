use crate::{
    fs, config::Config, error::Result,
    exec::{
        dialog_info::DialogInfo, dialog_confirm::DialogConfirm,
        restic_create::ResticCreate, restic_restore::ResticRestore
    }
};


/// Restores the latest archive which contains both tag "backup" and the managed path from the configuration
pub struct Restore {
    /// The config
    config: Config
}
impl Restore {
    /// Creates a command to push a new backup
    pub const fn new(config: Config) -> Self {
        Self { config }
    }

    /// Executes the command
    pub fn exec(self) -> Result {
        // Create a snapshot
        if fs::dir_exists(&self.config.restic.dir)? && self.config.restic.safe_restore {
            DialogInfo::new("Creating snapshot...")?.exec()?;
            ResticCreate::new(&self.config, ["snapshot"])?.exec()?;
        }

        // Remove all contents from the target directory
        if fs::dir_exists(&self.config.restic.dir)? && !fs::dir_is_empty(&self.config.restic.dir)? {
            // Ask for confirmation
            DialogConfirm::new(
                format!("Really delete everything within \"{}\" before restoration?", self.config.restic.dir),
                "Delete everything and restore", "Cancel"
            )?.exec()?;

            // Empty the directory
            DialogInfo::new(format!("Deleting everything within {}...", self.config.restic.dir))?.exec()?;
            fs::clear_dir(&self.config.restic.dir)?;
        }
        
        // Restore the backup
        DialogInfo::new("Restoring backup...")?.exec()?;
        ResticRestore::new(&self.config, ["backup"])?.exec()
    }
}
