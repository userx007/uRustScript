use interfaces::{Item, TokenType, Validator};
use std::collections::HashMap;
use std::error::Error;
use std::fmt;

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

    pub fn extract_macros(&self, items: &Vec<Item>, macros: &mut HashMap<String, String>) -> bool {
        true
    }

    pub fn replace_macros_in_item(
        &self,
        item: &mut Item,
        macros: &HashMap<String, String>,
    ) -> bool {
        true
    }

    pub fn replace_macros(&self, items: &mut Vec<Item>) -> bool {
        let mut macros: HashMap<String, String> = HashMap::new();

        if self.extract_macros(items, &mut macros) {
            for item in items {
                if !self.replace_macros_in_item(item, &macros) {
                    return false;
                }
            }
        } else {
            return false;
        }
        true
    }
}

impl Validator for ScriptValidator {
    fn validate_items(&self, items: &mut Vec<Item>) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}
