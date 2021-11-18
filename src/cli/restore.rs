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
        // Check if one of the target directories exist
        let mut exists = false;
        for dir in self.config.restic.dirs.iter() {
            exists |= fs::dir_exists(dir)?;
        }

        // Create a snapshot if necessary
        if exists && self.config.restic.safe_restore {
            // Ensure that *all* directories exist
            for dir in self.config.restic.dirs.iter() {
                fs::dir_create(dir)?;
            }

            // Create the snapshot
            DialogInfo::new("Creating snapshot...")?.exec()?;
            ResticCreate::new(&self.config, &["snapshot"])?.exec()?;
        }

        // Ask for confirmation before deletion
        if exists {
            let dirs = self.config.restic.dirs.join("\n");
            DialogConfirm::new(
                format!("Really delete everything within the following directories before restoration?\n\n{}", dirs),
                "Delete everything and restore", "Cancel"
            )?.exec()?;
        }

        // Remove all contents from the target directories
        for dir in self.config.restic.dirs.iter() {
            if fs::dir_exists(dir)? && !fs::dir_is_empty(dir)? {
                DialogInfo::new(format!("Deleting everything within {}...", dir))?.exec()?;
                fs::clear_dir(dir)?;
            }
        }

        // Restore the backup
        DialogInfo::new("Restoring backup...")?.exec()?;
        ResticRestore::new(&self.config, &["backup"])?.exec()
    }
}
