use interfaces::{Item, Parser, Reader, Validator};
use parser::ScriptParser;
use reader::ScriptReader;
use std::error::Error;
use validator::ScriptValidator;

fn main() -> Result<(), Box<dyn Error>> {
    let mut items = Vec::<Item>::new();

    let reader = ScriptReader::new();
    let parser = ScriptParser::new();
    let validator = ScriptValidator::new();

    reader.read_script("script.txt", &mut items)?;
    parser.parse_script(&mut items)?;
    validator.validate_items(&mut items)?;

    /*
        for item in &items {
            println!("{:?}", item);
        }
    */

    Ok(())
}
