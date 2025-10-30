use std::error::Error;
use interfaces::{Reader, Validator};

pub struct ScriptValidator;

impl ScriptValidator {
    pub fn new() -> Self {
        ScriptValidator{}
    }
}

impl Validator for ScriptValidator {
    fn validate_script(&self, input : &Vec<String>) -> Result<(), Box<dyn Error>> {
        println!("\nValidating...\n");
        for line in input {
            println!("Line: [{}] -> ok", line)
        }
        Ok(())
    }
}

