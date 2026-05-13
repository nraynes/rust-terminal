use crate::Command;

/// Terminal construct for working with cli commands.
pub struct Terminal {}

impl Terminal {
    pub fn command() -> Command {
        Command::new()
    }
}
