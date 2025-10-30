use std::error::Error;
use interfaces::{Reader, Validator, Parser};
use reader::ScriptReader;
use validator::ScriptValidator;
use parser::ScriptParser;

fn main() -> Result<(), Box<dyn Error>> {
    let mut lines = Vec::new();

    let reader = ScriptReader::new();
    let validator = ScriptValidator::new();
    let parser = ScriptParser::new();

    reader.read_script("script.txt", &mut lines)?;
    validator.validate_script(&lines)?;
    parser.parse_script(&lines)?;

    Ok(())
}
