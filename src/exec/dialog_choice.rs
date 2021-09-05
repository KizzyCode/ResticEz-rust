use crate::{ error::Result, exec::GenericExecutable };


/// A choice dialogue
#[derive(Debug)]
pub struct DialogChoice {
    /// The underlying generic executable
    exec: GenericExecutable
}
impl DialogChoice {
    /// Creates a new choice dialog command
    pub fn new<M, O, OSA, OSB>(message: M, options: O) -> Result<Self>
        where M: ToString, O: IntoIterator<Item = (OSA, OSB)>, OSA: ToString, OSB: ToString
    {
        // Build the CLI options
        let mut cli_options = vec![
            "--clear".to_string(), "--stdout".to_string(),
            "--menu".to_string(), message.to_string(), "0".to_string(), "0".to_string(), "0".to_string()
        ];
        for (tag, desc) in options {
            cli_options.push(tag.to_string());
            cli_options.push(desc.to_string());
        }

        // Create the executable
        let dialog = GenericExecutable::find("dialog")?;
        let exec = GenericExecutable::new(dialog, cli_options);
        Ok(Self { exec })
    }
    
    /// Shows the menu dialog and captures the input as string
    pub fn exec(self) -> Result<String> {
        self.exec.exec_stringout()
    }
}
