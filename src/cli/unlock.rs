use crate::cli::Command;
use crate::config::Config;
use crate::error::Error;
use crate::exec::Exec;
use crate::exec::dialog_confirm::DialogConfirm;
use crate::exec::dialog_info::DialogInfo;
use crate::exec::restic_break_lock::ResticBreakLock;

/// Removes a stale lock on the restic repository
#[derive(Debug)]
pub struct Unlock {
    /// The config
    config: Config,
}
impl Command for Unlock {
    fn new(config: Config) -> Self {
        Self { config }
    }

    fn exec(self) -> Result<(), Error> {
        DialogConfirm::new("Do you really want to break any stale lock?", "Break lock", "Cancel")?.exec()?;
        DialogInfo::new("Breaking lock...")?.exec()?;
        ResticBreakLock::new(&self.config)?.exec()
    }
}
