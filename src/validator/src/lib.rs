use std::collections::HashSet;
use std::error::Error;
use std::fmt;

use interfaces::{Item, TokenType, Validator};


#[derive(Debug)]
enum ValidateError {
    LoadedPlugins,
}

impl fmt::Display for ValidateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ValidateError::LoadedPlugins => write!(f, "Needed plugins not loaded"),
        }
    }
}

impl Error for ValidateError {}

pub struct ScriptValidator;

impl ScriptValidator {
    pub fn new() -> Self {
        ScriptValidator {}
    }

    fn validate_plugins_availability(&self, items: &mut Vec<Item>) -> bool {
        let mut loaded = HashSet::new();
        let mut used   = HashSet::new();

        for item in items {
            match &item.token_type {
                TokenType::LoadPlugin{ plugin, .. } => {
                    loaded.insert(plugin);
                }
                TokenType::VariableMacro{ plugin, .. } => {
                    used.insert(plugin);
                }
                TokenType::Command{ plugin, .. } => {
                    used.insert(plugin);
                }
                _ => {}
            }
        }

        println!("Loaded: {:?}", loaded);
        println!("Used  : {:?}", used);

        if loaded != used {
            let missing: HashSet<_> = used.difference(&loaded).cloned().collect();
            println!("Missing  : {:?}", missing);
            return false;
        }
        true
    }

//    fn load_plugins(&self, )
}

impl Validator for ScriptValidator {
    fn validate_script(&self, items: &mut Vec<Item>) -> Result<(), Box<dyn Error>> {
        println!("Validating script ...");
        if false == self.validate_plugins_availability(items) {
            return Err(Box::new(ValidateError::LoadedPlugins));
        }
        Ok(())
    }
}
