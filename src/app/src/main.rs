use std::error::Error;

use interfaces::{Item, Parser, Reader, Runner, Validator};
use parser::ScriptParser;
use reader::ScriptReader;
use runner::ScriptRunner;
use validator::ScriptValidator;

fn main() -> Result<(), Box<dyn Error>> {
    let mut items = Vec::<Item>::new();

    let reader = ScriptReader::new();
    let mut parser = ScriptParser::new();
    let validator = ScriptValidator::new();
    let runner = ScriptRunner::new();

    reader.read_script("script.txt", &mut items)?;
    parser.parse_script(&mut items)?;
    validator.validate_script(&mut items)?;
    runner.run_script(&mut items)?;

    Ok(())
}
