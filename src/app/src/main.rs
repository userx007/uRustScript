use std::error::Error;
use interfaces::{Reader, Validator};
use reader::ScriptReader;
use validator::ScriptValidator;



fn main() -> Result<(), Box<dyn Error>> {
    let reader = ScriptReader::new();
    let mut lines = Vec::new();

    reader.read_script("script.txt", &mut lines)?;
    for line in &lines {
        println!("{}", line);
    }

    let validator = ScriptValidator::new();
    validator.validate_script(&lines)?;

    Ok(())
}

