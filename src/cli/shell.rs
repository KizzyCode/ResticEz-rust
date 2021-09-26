use crate::{ config::Config, error::Result, exec::shell_session::ShellSession };


/// Creaxtes a new shell session with the restic config copied to env
pub struct Shell {
    /// The config
    config: Config
}
impl Shell {
    /// Creates a shell session with the restic config copied to env
    pub const fn new(config: Config) -> Self {
        Self { config }
    }

    /// Starts the shell
    pub fn exec(self) -> Result {
        ShellSession::new(&self.config)?.exec()
    }
}
