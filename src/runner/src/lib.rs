use interfaces::{Item, TokenType, Runner};
use std::error::Error;
use std::fmt;

#[derive(Debug)]
enum RunError {
    InvalidStatement,
}

impl fmt::Display for RunError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RunError::InvalidStatement => write!(f, "Invalid item in script"),
        }
    }
}

impl Error for RunError {}

pub struct ScriptRunner;

impl ScriptRunner {
    pub fn new() -> Self {
        ScriptRunner {}
    }
}

impl Runner for ScriptRunner {
    fn run_script(&self, items: &mut Vec<Item>) -> Result<(), Box<dyn Error>> {
        println!("Running script ...");
        Ok(())
    }
}
