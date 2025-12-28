use crate::cli::Command;
use crate::config::Config;
use crate::error::Error;
use crate::exec::Exec;
use crate::exec::shell_session::ShellSession;

/// Creaxtes a new shell session with the restic config copied to env
#[derive(Debug)]
pub struct Shell {
    /// The config
    config: Config,
}
impl Command for Shell {
    fn new(config: Config) -> Self {
        Self { config }
    }

    fn exec(self) -> Result<(), Error> {
        ShellSession::new(&self.config)?.exec()
    }
}
