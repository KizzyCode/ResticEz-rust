use crate::{ config::Config, error::Result };
use ezexec::ExecBuilder;


/// Opens a `tmux` session with the given environment set
#[derive(Debug)]
pub struct TmuxSession {
    /// The underlying raw command
    exec: ExecBuilder
}
impl TmuxSession {
    /// Creates a new `tmux` session with `config` as environment
    pub fn new(config: &Config) -> Result<Self> {
        // Create the tmux command
        let mut exec = ExecBuilder::with_name("tmux", Vec::<String>::new())?;
        exec.set_envs(config.to_env()?);

        Ok(Self { exec })
    }
    
    /// Starts tmux
    pub fn exec(self) -> Result {
        Ok(self.exec.spawn_transparent()?.wait()?)
    }
}
