use crate::{ config::Config, error::Result, exec::tmux_session::TmuxSession };


/// Creaxtes a tmux shell with the restic config as environment
pub struct Tmux {
    /// The config
    config: Config
}
impl Tmux {
    /// Creates a tmux shell with the restic config as environment
    pub const fn new(config: Config) -> Self {
        Self { config }
    }

    /// Starts the shell
    pub fn exec(self) -> Result {
        TmuxSession::new(&self.config)?.exec()
    }
}
