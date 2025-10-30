use interfaces::{Parser, Reader, Validator, Item};
use parser::ScriptParser;
use reader::ScriptReader;
use std::error::Error;
use validator::ScriptValidator;

fn main() -> Result<(), Box<dyn Error>> {
    let mut items = Vec::<Item>::new();

    let reader = ScriptReader::new();
    let validator = ScriptValidator::new();
    let parser = ScriptParser::new();

    reader.read_script("script.txt", &mut items)?;
    validator.validate_script(&mut items)?;
    parser.parse_script(&mut items)?;

    Ok(())
}
