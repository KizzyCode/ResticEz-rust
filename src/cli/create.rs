use crate::cli::Command;
use crate::config::Config;
use crate::error::Error;
use crate::exec::Exec;
use crate::exec::dialog_info::DialogInfo;
use crate::exec::restic_create::ResticCreate;

/// Creates a new backup
#[derive(Debug)]
pub struct Create {
    /// The config
    config: Config,
}
impl Command for Create {
    fn new(config: Config) -> Self {
        Self { config }
    }

    fn exec(self) -> Result<(), Error> {
        DialogInfo::new("Creating backup...")?.exec()?;
        ResticCreate::new(&self.config, &["backup"])?.exec()
    }
}
