use crate::config::Config;
use crate::error::Result;
use ezexec::lookup::Shell;
use ezexec::ExecBuilder;

/// Opens a `shell` session with the given environment set
#[derive(Debug)]
pub struct ShellSession {
    /// The underlying raw command
    exec: ExecBuilder,
}
impl ShellSession {
    /// Creates a new `shell` session with `config` as environment
    pub fn new(config: &Config) -> Result<Self> {
        // Create the shell command and define a PS1 string
        let shell = Shell::find()?;
        let mut exec = ExecBuilder::with_path(&shell, Vec::<String>::new())?;
        exec.set_envs(config.to_env()?);

        Ok(Self { exec })
    }

    /// Creates the shell session
    pub fn exec(self) -> Result {
        eprintln!();
        eprintln!("======= Spawning new subshell: =======");
        Ok(self.exec.spawn_transparent()?.wait()?)
    }
}
