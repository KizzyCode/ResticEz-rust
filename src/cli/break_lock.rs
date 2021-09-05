use crate::{
    config::Config, error::Result,
    exec::{ dialog_confirm::DialogConfirm, dialog_info::DialogInfo, restic_break_lock::ResticBreakLock }
};


/// Removes a stale lock on the restic repository
pub struct BreakLock {
    /// The config
    config: Config
}
impl BreakLock {
    /// Creates a command to remove a stale lock on the restic repository
    pub const fn new(config: Config) -> Self {
        Self { config }
    }

    /// Executes the command
    pub fn exec(self) -> Result {
        DialogConfirm::new("Do you really want to break any stale lock?", "Break lock", "Cancel")?.exec()?;
        DialogInfo::new("Breaking lock...")?.exec()?;
        ResticBreakLock::new(&self.config)?.exec()
    }
}