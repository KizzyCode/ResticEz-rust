use crate::{
    config::Config, error::Result,
    exec::{ dialog_info::DialogInfo, restic_create::ResticCreate }
};


/// Creates a new backup
pub struct Create {
    /// The config
    config: Config
}
impl Create {
    /// Creates a command to push a new backup
    pub const fn new(config: Config) -> Self {
        Self { config }
    }

    /// Executes the command
    pub fn exec(self) -> Result {
        DialogInfo::new("Creating backup...")?.exec()?;
        ResticCreate::new(&self.config, &["backup"])?.exec()
    }
}
