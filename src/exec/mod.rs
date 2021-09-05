pub mod dialog_choice;
pub mod dialog_confirm;
pub mod dialog_creds;
pub mod dialog_info;
pub mod restic_break_lock;
pub mod restic_check;
pub mod restic_create;
pub mod restic_list;
pub mod restic_restore;
pub mod restic_prune;
pub mod tmux_session;
pub mod shell_command;

use crate::error::Result;
use std::{ env, process::Command };


/// An executable command
#[derive(Debug)]
pub struct GenericExecutable {
    /// The binary to execute
    binary: String,
    /// The arguments to pass to the binary
    args: Vec<String>,
    /// The environment variables to set
    envs: Vec<(String, String)>
}
impl GenericExecutable {
    /// Checks if a binary exists in one of the binary `PATH`s
    pub fn find<B>(binary: B) -> Result<String> where B: AsRef<str> {
        // Get the PATH environment or fallback to a reasonable default
        let paths = env::var("PATH")
            .map(|p| env::split_paths(&p).collect::<Vec<_>>())
            .unwrap_or(vec!["/bin".into(), "/usr/bin".into(), "/usr/local/bin".into()]);

        // Create the full path and test it
        let binary = binary.as_ref();
        for path in paths {
            let bin_path = path.join(binary);
            if bin_path.is_file() {
                // Convert the path to a string
                let bin_path_string = bin_path.to_str()
                    .ok_or(eexec!("Cowardly refusing to use non-UTF-8 path: {}", bin_path.to_string_lossy()))?;
                return Ok(bin_path_string.to_string())
            }
        }
        Err(eexec!("`{}` not in PATH", binary))
    }

    /// Creates a new CLI process with the given binary
    pub fn new<B, A, AS>(binary: B, args: A) -> Self where B: ToString, A: IntoIterator<Item = AS>, AS: ToString {
        Self {
            binary: binary.to_string(),
            args: args.into_iter().map(|a| a.to_string()).collect(),
            envs: Vec::new()
        }
    }
    /// Sets the environment variables
    pub fn set_envs<I, K, V>(&mut self, vars: I) where I: IntoIterator<Item = (K, V)>, K: ToString, V: ToString {
        vars.into_iter().for_each(|(k, v)| {
            let key = k.to_string();
            let value = v.to_string();
            self.envs.push((key, value));
        })
    }
    
    /// Runs the executable
    pub fn exec(self) -> Result {
        // Execute process
        let exec_result = Command::new(&self.binary)
            .args(self.args)
            .envs(self.envs)
            .current_dir("/")
            .status();
        let exit_status = match exec_result {
            Ok(exit_status) => exit_status,
            Err(e) => Err(eexec!("Failed to execute `{}` ({})", self.binary, e))?
        };
        
        // Process result
        if !exit_status.success() {
            let code = exit_status.code().unwrap_or(-1);
            Err(eexec!("`{}` exited with {}: <stderr not captured>", self.binary, code))?
        }
        Ok(())
    }
    /// Runs the executable command and captures stdout as string
    pub fn exec_stringout(self) -> Result<String> {
        // Execute process
        let exec_result = Command::new(&self.binary)
            .args(self.args)
            .envs(self.envs)
            .current_dir("/")
            .output();
        let output = match exec_result {
            Ok(exit_status) => exit_status,
            Err(e) => Err(eexec!("Failed to execute `{}` ({})", self.binary, e))?
        };
        
        // Process result
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        if !output.status.success() {
            let code = output.status.code().unwrap_or(-1);
            Err(eexec!("`{}` exited with {}: {}", self.binary, code, stderr))?
        }
        Ok(String::from_utf8_lossy(&output.stdout).into())
    }
}
