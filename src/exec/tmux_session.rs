use crate::{ config::Config, error::Result, exec::GenericExecutable };


/// Opens a `tmux` session with the given environment set
#[derive(Debug)]
pub struct TmuxSession {
    /// The underlying raw command
    exec: GenericExecutable
}
impl TmuxSession {
    /// Creates a new `tmux` session with `config` as environment
    pub fn new(config: &Config) -> Result<Self> {
        // Create the tmux command
        let tmux = GenericExecutable::find("tmux")?;
        let mut exec = GenericExecutable::new(tmux, Vec::<String>::new());
        exec.set_envs(config.to_env()?);

        Ok(Self { exec })
    }
    
    /// Starts tmux
    pub fn exec(self) -> Result {
        self.exec.exec()
    }
}
