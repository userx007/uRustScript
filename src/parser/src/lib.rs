use interfaces::Parser;
use std::error::Error;

pub struct ScriptParser;

impl ScriptParser {
    pub fn new() -> Self {
        ScriptParser {}
    }
}

impl Parser for ScriptParser {
    fn parse_script(&self, input: &Vec<String>) -> Result<(), Box<dyn Error>> {
        println!("Parsing script ...");
        for line in input {
            println!("\tParsing line {}", line);
        }
        Ok(())
    }
}