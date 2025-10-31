use std::collections::HashMap;
use std::error::Error;
use std::fmt;

use interfaces::{Item, TokenType, Validator};
use utils::replacer;

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

    fn extract_macros(&self, items: &Vec<Item>, macros: &mut HashMap<String, String>) {
        for item in items {
            if let TokenType::ConstantMacro{name, value} = &item.token_type {
                macros.insert(name.to_string(), value.to_string());
            }
        }
        // sort
    }


    fn replace_macros_in_item(
        &self,
        item: &mut Item,
        macros: &HashMap<String, String>,
    ) -> bool {
        true
    }



    pub fn replace_macros(&self, items: &mut Vec<Item>) -> bool {
        let mut macros: HashMap<String, String> = HashMap::new();

        self.extract_macros(items, &mut macros);
        //println!("{:?}", macros);

        for item in items {
            if !self.replace_macros_in_item(item, &macros) {
                return false;
            }
        }
        true
    }
}

impl Validator for ScriptValidator {
    fn validate_script(&self, items: &mut Vec<Item>) -> Result<(), Box<dyn Error>> {
        println!("Validating script ...");
        if self.replace_macros(items){
            ;
        }
        Ok(())
    }
}
