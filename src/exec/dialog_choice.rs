use crate::error::Result;
use ezexec::ExecBuilder;
use std::convert::TryInto;


/// A choice dialogue
#[derive(Debug)]
pub struct DialogChoice {
    /// The underlying generic executable
    exec: ExecBuilder
}
impl DialogChoice {
    /// Creates a new choice dialog command
    pub fn new<M, O, OSA, OSB>(message: M, options: O) -> Result<Self>
        where M: ToString, O: IntoIterator<Item = (OSA, OSB)>, OSA: ToString, OSB: ToString
    {
        // Build the CLI options
        let mut args = vec![
            "--clear".to_string(), "--stdout".to_string(),
            "--menu".to_string(), message.to_string(), "0".to_string(), "0".to_string(), "0".to_string()
        ];
        for (tag, desc) in options {
            args.push(tag.to_string());
            args.push(desc.to_string());
        }

        // Create the executable
        Ok(Self { exec: ExecBuilder::with_name("dialog", args)? })
    }
    
    /// Shows the menu dialog and captures the input as string
    pub fn exec(self) -> Result<String> {
        Ok(self.exec.spawn_captured()?.try_into()?)
    }
}
