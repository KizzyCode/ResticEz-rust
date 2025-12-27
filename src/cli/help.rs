/// A command to display the help
pub struct Help;
impl Help {
    /// The help text
    const HELP_TEXT: &'static str = include_str!("../../HELP.txt");

    /// Creates a new command to display the help
    pub fn new() -> Self {
        Self
    }

    /// Executes the command
    pub fn display(self) {
        println!("{}", Self::HELP_TEXT);
    }
}
