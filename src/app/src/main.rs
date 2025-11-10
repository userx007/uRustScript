use std::error::Error;

use interfaces::Item;
use parser::ScriptParser;
use plugin_manager::PluginManager;
use reader::ScriptReader;
use runner::ScriptRunner;
use validator::ScriptValidator;

const SCRIPT_NAME: &'static str = "script.txt";
const INI_NAME: &'static str = "settings.ini";
const PLUGINS_PATH: &'static str = "target/debug";

fn main() -> Result<(), Box<dyn Error>> {
    let mut items = Vec::<Item>::new();

    let reader = ScriptReader::new(SCRIPT_NAME);
    let mut parser = ScriptParser::new();
    let validator = ScriptValidator::new();
    let mut runner = ScriptRunner::new(INI_NAME);
    let mut plugin_manager = PluginManager::new(PLUGINS_PATH);

    reader.read_script(&mut items)?;
    parser.parse_script(&mut items)?;
    validator.validate_script(&mut items, &mut plugin_manager)?;
    runner.run_script(&mut items, &mut plugin_manager)?;

    for item in items {
        println!("{:?}", item);
    }

    Ok(())
}
