use std::collections::HashSet;
use std::error::Error;
use std::fmt;

use interfaces::{Item, TokenType, Validator};


#[derive(Debug)]
enum ValidateError {
    InvalidStatement,
}

impl fmt::Display for ValidateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ValidateError::InvalidStatement => write!(f, "Invalid item in script"),
        }
    }
}

impl Error for ValidateError {}

pub struct ScriptValidator;

impl ScriptValidator {
    pub fn new() -> Self {
        ScriptValidator {}
    }

    fn validate_plugins(&self, items: &mut Vec<Item>) -> bool {
        let mut loaded = HashSet::new();
        let mut used   = HashSet::new();

        for item in items {
            match &item.token_type {
                TokenType::LoadPlugin{ name, .. } => {
                    loaded.insert(name);
                }
                TokenType::VariableMacro{ plugin, .. } => {
                    used.insert(plugin);
                }
                _ => {}
            }
        }
        println!("Loaded: {:?}", loaded);
        println!("Used  : {:?}", used);
        true
    }
}

impl Validator for ScriptValidator {
    fn validate_script(&self, items: &mut Vec<Item>) -> Result<(), Box<dyn Error>> {
        println!("Validating script ...");
        self.validate_plugins(items);
        Ok(())
    }
}
