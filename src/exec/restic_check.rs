use crate::config::Config;
use crate::error::Result;
use ezexec::ExecBuilder;

/// Checks the repository consistency
#[derive(Debug)]
pub struct ResticCheck {
    /// The underlying generic executable
    exec: ExecBuilder,
}
impl ResticCheck {
    /// Creates a command to check the repository consistency
    pub fn new(config: &Config) -> Result<Self> {
        let mut exec = ExecBuilder::with_name("restic", ["check", "--read-data"])?;
        exec.set_envs(config.to_env()?);

        Ok(Self { exec })
    }

    /// Checks the repository consistency
    pub fn exec(self) -> Result {
        Ok(self.exec.spawn_transparent()?.wait()?)
    }
}
