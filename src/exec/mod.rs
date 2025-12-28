pub mod dialog_choice;
pub mod dialog_confirm;
pub mod dialog_creds;
pub mod dialog_info;
pub mod restic_break_lock;
pub mod restic_check;
pub mod restic_create;
pub mod restic_list;
pub mod restic_prune;
pub mod restic_restore;
pub mod shell_command;
pub mod shell_session;

use crate::error::Error;
use std::path::Path;
use std::process::{Command, Stdio};
use std::{env, iter};

/// An executable step
pub trait Exec {
    /// The success-output value
    type Output;

    /// Executes the step
    fn exec(self) -> Result<Self::Output, Error>;
}

/// Extension traits for [`Command`]
pub(in crate::exec) trait CommandExt {
    /// Drives `self` to completion and expects an exit-status `0`
    fn exit0(&mut self) -> Result<(), Error>;
    /// Drives `self` to completion and expects an exit-status `0` similar to [`CommandExt::exit0`], but captures stdout
    fn stdout0(&mut self) -> Result<String, Error>;
    /// The command invocation as human readable debug string
    ///
    /// # Important
    /// This string is a debug-approximation for human consumption; it is _not_ an accurate 1:1 representation of the
    /// command invocation.
    fn as_debug_string(&self) -> String;
}
impl CommandExt for Command {
    fn exit0(&mut self) -> Result<(), Error> {
        let status = self.status().map_err(|e| eexec!("Failed to execute command: {e}"))?;
        match status.code() {
            Some(0) => Ok(()),
            Some(code) => Err(eexec!("Command failed with exit code {code}: {}", self.as_debug_string())),
            None => Err(eexec!("Command exited with unknown exit code: {}", self.as_debug_string())),
        }
    }

    fn stdout0(&mut self) -> Result<String, Error> {
        // Explicitly set stderr to inherited to ensure it will not be captured by `[Child::output]`
        self.stderr(Stdio::inherit());
        let output = self.output().map_err(|e| eexec!("Failed to execute command: {e}"))?;

        // Capture and retain stdout and validate status code
        let stdout = String::from_utf8(output.stdout).map_err(|e| einval!("Invalid UTF-8 output in stdout: {e}"))?;
        match output.status.code() {
            Some(0) => Ok(stdout),
            Some(code) => Err(eexec!("Command failed with exit code {code}: {}", self.as_debug_string())),
            None => Err(eexec!("Command exited with unknown exit code: {}", self.as_debug_string())),
        }
    }

    fn as_debug_string(&self) -> String {
        // Get all command components
        iter::once(self.get_program().display())
            // Append arguments
            .chain(self.get_args().map(|arg| arg.display()))
            // Convert everything to a string
            .map(|component| component.to_string())
            // Collect all arguments to join them later
            .collect::<Vec<_>>()
            // Join all arguments
            .join(" ")
    }
}

/// OS-specific extension trait for [`Command`]
pub trait CommandOsExt {
    /// Gets the current system shell binary
    fn shell() -> Result<String, Error>;

    /// Executes the given script in the OS-specific shell, and expects an exit-status `0` and captures stdout
    ///
    /// # Warning
    /// As always, treat raw shell commands with caution and never pass any untrusted input into the invocation.
    /// Escaping of malicious input is extremely hard to do correctly, and should not be attempted.
    fn shell_stdout0(script: &str) -> Result<String, Error>;
}
impl CommandOsExt for Command {
    fn shell() -> Result<String, Error> {
        if cfg!(target_family = "unix") {
            // Use `$SHELL`, or fallback to `$PATH`s `sh` on unix-likes
            let shell = env::var("SHELL").unwrap_or_else(|_| "sh".to_string());
            Ok(shell)
        } else if cfg!(target_family = "windows") {
            // Use the powershell on windows
            Ok("powershell.exe".to_string())
        } else {
            // We cannot estimate what OS we are on, and how the shell works here, so raise an error
            Err(eexec!("Unable to invoke the user command shell on this platform"))
        }
    }

    fn shell_stdout0(script: &str) -> Result<String, Error> {
        // Determine shell
        let shell = Self::shell()?;
        let shell_name = Path::new(&shell).file_name()
            // Convert file name to string...
            .and_then(|shell_name| shell_name.to_str())
            // ... or fallback to the whole shell
            .unwrap_or(&shell);

        // Assemble shell command
        match shell_name {
            "sh" | "bash" | "zsh" => Command::new(shell).arg("-c").arg(script).stdout0(),
            "powershell.exe" => Command::new(shell).arg("-Command").arg(script).stdout0(),
            shell => Err(eexec!("Unable to invoke the user command shell on this platform: {shell}")),
        }
    }
}
