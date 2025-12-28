pub mod check;
pub mod create;
pub mod help;
pub mod list;
pub mod prune;
pub mod restore;
pub mod shell;
pub mod unlock;

use crate::cli::check::Check;
use crate::cli::create::Create;
use crate::cli::help::Help;
use crate::cli::list::List;
use crate::cli::prune::Prune;
use crate::cli::restore::Restore;
use crate::cli::shell::Shell;
use crate::cli::unlock::Unlock;
use crate::config::Config;
use crate::error::Error;
use crate::exec::Exec;
use crate::exec::dialog_choice::DialogChoice;
use crate::exec::shell_command::ShellCommand;
use std::iter::Peekable;
use std::mem;

/// An executable command
pub trait Command {
    /// Creates a new command
    fn new(config: Config) -> Self;

    /// Executes the command
    fn exec(self) -> Result<(), Error>;
}

/// A CLI command processor
#[derive(Debug)]
pub struct CliCommand {
    /// The verb
    verb: Option<String>,
    /// The config
    config: Config,
}
impl CliCommand {
    /// Creates a new CLI command processor
    pub fn new<Argv>(config: Config, argv: &mut Peekable<Argv>) -> Self
    where
        Argv: Iterator<Item = String>,
    {
        Self { verb: argv.next(), config }
    }

    /// Executes the CLI command
    pub fn exec(mut self) -> Result<(), Error> {
        // Select the verb
        #[rustfmt::skip]
        let verb = match self.verb.take() {
            Some(verb) => verb,
            None => DialogChoice::new("Please select an action:", [
                // Ask the user to select the action manually
                ("create", "Creates a new archive"),
                ("list", "Lists the existing archives"),
                ("restore", "Restores the latest archiv tagged with \"backup\""),
                ("shell", "Opens a shell session configured for easy manual restic invocation"),
                ("check", "Checks the repository for consistency"),
                ("prune", "Removes all unused chunks from the repository"),
                ("unlock", "Breaks a stale lock"),
            ])?.exec()?
        };

        // Execute the appropriate command
        match verb.as_str() {
            "break-lock" | "unlock" => self._exec::<Unlock>(),
            "check" | "test" => self._exec::<Check>(),
            "create" | "backup" => self._exec::<Create>(),
            "list" | "ls" => self._exec::<List>(),
            "prune" => self._exec::<Prune>(),
            "restore" => self._exec::<Restore>(),
            "shell" | "sh" => self._exec::<Shell>(),
            "help" => Ok(Help::new().display()),
            verb => {
                // Display help and yield error
                Help::new().display();
                Err(einval!("Invalid verb: {verb}"))
            }
        }
    }

    /// Executes a command step
    fn _exec<CommandImpl>(mut self) -> Result<(), Error>
    where
        CommandImpl: Command,
    {
        // Take preexec/postexec to avoid move issues
        let commands_preexec = mem::take(&mut self.config.commands.preexec);
        let commands_postexec = mem::take(&mut self.config.commands.postexec);

        // Execute pre-exec commands
        for preexec in commands_preexec {
            // Execute command in shell
            ShellCommand::new(preexec)?.exec()?;
        }

        // Execute command
        let command_impl = CommandImpl::new(self.config);
        command_impl.exec()?;

        // Execute post-exec commands
        for postexec in commands_postexec {
            // Execute command in shell
            ShellCommand::new(postexec)?.exec()?;
        }
        Ok(())
    }
}
