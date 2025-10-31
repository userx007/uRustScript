use std::collections::HashMap;
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
}

impl Validator for ScriptValidator {
    fn validate_script(&self, items: &mut Vec<Item>) -> Result<(), Box<dyn Error>> {
        println!("Validating script ...");
        Ok(())
    }
}
