use crate::config::Config;
use crate::error::Error;
use crate::exec::dialog_info::DialogInfo;
use crate::exec::restic_create::ResticCreate;

/// Creates a new backup
pub struct Create {
    /// The config
    config: Config,
}
impl Create {
    /// Creates a command to push a new backup
    pub const fn new(config: Config) -> Self {
        Self { config }
    }

    /// Executes the command
    pub fn exec(self) -> Result<(), Error> {
        DialogInfo::new("Creating backup...")?.exec()?;
        ResticCreate::new(&self.config, &["backup"])?.exec()
    }
}
