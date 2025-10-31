use std::error::Error;

use interfaces::{Item, Reader, Parser, Validator, Runner};
use reader::ScriptReader;
use parser::ScriptParser;
use validator::ScriptValidator;
use runner::ScriptRunner;

fn main() -> Result<(), Box<dyn Error>> {
    let mut items = Vec::<Item>::new();

    let reader = ScriptReader::new();
    let parser = ScriptParser::new();
    let validator = ScriptValidator::new();
    let runner = ScriptRunner::new();

    reader.read_script("script.txt", &mut items)?;
    parser.parse_script(&mut items)?;
    validator.validate_script(&mut items)?;
    runner.run_script(&mut items)?;

    /*
        for item in &items {
            println!("{:?}", item);
        }
    */

    Ok(())
}
