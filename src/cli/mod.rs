pub mod break_lock;
pub mod check;
pub mod create;
pub mod help;
pub mod list;
pub mod prune;
pub mod restore;
pub mod tmux;

use crate::{
    config::Config, error::Result, exec::dialog_choice::DialogChoice,
    cli::{
        break_lock::BreakLock, check::Check, create::Create, help::Help,
        list::List, prune::Prune, restore::Restore, tmux::Tmux
    }
};
use std::env;


/// A CLI command processor
#[derive(Debug)]
pub struct CliCommand {
    /// The verb
    verb: Option<String>,
    /// The config
    config: Config
}
impl CliCommand {
    /// Creates a new CLI command processor
    pub fn new(config: Config) -> Self {
        Self { verb: env::args().skip(1).next(), config }
    }

    /// Executes the CLI command
    pub fn exec(mut self) -> Result {
        // Select the verb
        if self.verb.is_none() {
            let verbs = [
                ("create", "Creates a new archive"),
                ("list", "Lists the existing archives"),
                ("restore", "Restores the latest archiv tagged with \"backup\""),
                ("tmux", "Opens a tmux session configured for easy manual restic invocation"),
                ("check", "Checks the repository for consistency"),
                ("prune", "Removes all unused chunks from the repository"),
                ("break-lock", "Breaks a stale lock")
            ];
            self.verb = Some(DialogChoice::new("Please select an action:", verbs)?.exec()?);
        }

        // Execute the appropriate command
        match self.verb.expect("Failed to unwrap verb?!").as_str() {
            "break-lock" => BreakLock::new(self.config).exec(),
            "check" | "test" => Check::new(self.config).exec(),
            "create" | "backup" => Create::new(self.config).exec(),
            "list" | "ls" => List::new(self.config).exec(),
            "prune" => Prune::new(self.config).exec(),
            "restore" => Restore::new(self.config).exec(),
            "tmux" | "shell" | "sh" => Tmux::new(self.config).exec(),
            "help" => Ok(Help::new().display()),
            verb => {
                Help::new().display();
                Err(einval!("Invalid verb: {}", verb))
            }
        }
    }
}
