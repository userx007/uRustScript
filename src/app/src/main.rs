use interfaces::{Reader, Validator};
use reader::ScriptReader;
use std::error::Error;
use validator::ScriptValidator;

fn main() -> Result<(), Box<dyn Error>> {
    let mut lines = Vec::new();
    let reader = ScriptReader::new();
    let validator = ScriptValidator::new();

    reader.read_script("script.txt", &mut lines)?;
    validator.validate_script(&lines)?;

    Ok(())
}
