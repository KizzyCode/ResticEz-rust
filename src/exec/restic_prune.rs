use crate::config::Config;
use crate::error::Result;
use ezexec::ExecBuilder;

/// Prunes all unused chunks from the repository
#[derive(Debug)]
pub struct ResticPrune {
    /// The underlying generic executable
    exec: ExecBuilder,
}
impl ResticPrune {
    /// Creates a command to prune all unused chunks from the repository
    pub fn new(config: &Config) -> Result<Self> {
        let mut exec = ExecBuilder::with_name("restic", ["prune"])?;
        exec.set_envs(config.to_env()?);

        Ok(Self { exec })
    }

    /// Prunes all unused chunks from the repository
    pub fn exec(self) -> Result {
        Ok(self.exec.spawn_transparent()?.wait()?)
    }
}
