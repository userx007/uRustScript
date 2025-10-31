use interfaces::{Item, Reader, Parser};
use reader::ScriptReader;
use parser::ScriptParser;
use std::error::Error;


fn main() -> Result<(), Box<dyn Error>> {
    let mut items = Vec::<Item>::new();

    let reader = ScriptReader::new();
    let parser = ScriptParser::new();

    reader.read_script("script.txt", &mut items)?;
    parser.parse_script(&mut items)?;

    for item in &items {
        println!("{:?}", item);
    }

    Ok(())
}
