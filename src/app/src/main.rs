use std::error::Error;

use interfaces::Item;
use parser::ScriptParser;
use reader::ScriptReader;
use runner::ScriptRunner;
use validator::ScriptValidator;
use plugin_manager::PluginManager;

fn main() -> Result<(), Box<dyn Error>> {
    let mut items = Vec::<Item>::new();

    let reader = ScriptReader::new();
    let mut parser = ScriptParser::new();
    let validator = ScriptValidator::new();
    let runner = ScriptRunner::new();
    let mut plugin_manager = PluginManager::new();

    reader.read_script("script.txt", &mut items)?;
    parser.parse_script(&mut items)?;
    validator.validate_script(&mut items, &mut plugin_manager)?;
    runner.run_script(&mut items, &mut plugin_manager)?;

    Ok(())
}
