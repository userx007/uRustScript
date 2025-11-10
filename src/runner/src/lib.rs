use interfaces::{Item, TokenType};
use plugin_api::{plugin_do_dispatch, plugin_get_data, PluginHandle};
use plugin_manager::PluginManager;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use utils::string_replacer;

#[derive(Debug)]
enum RunError {
    ErrorExecutingCommand,
    PluginNotFound,
}

impl fmt::Display for RunError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for RunError {}

pub struct ScriptRunner {
    macros: HashMap<String, String>,
}

impl ScriptRunner {
    pub fn new() -> Self {
        ScriptRunner {
            macros: HashMap::new(),
        }
    }

    fn execute_plugin_command(
        &self,
        plugin_manager: &mut PluginManager,
        plugin: &str,
        command: &str,
        args: &mut String,
    ) -> Result<Option<String>, Box<dyn Error>> {
        let descriptor = plugin_manager
            .plugins
            .get(plugin)
            .ok_or_else(|| RunError::PluginNotFound)?;
        string_replacer::replace_macros(args, &self.macros);
        unsafe {
            let handle: &mut PluginHandle = &mut *descriptor.handle;

            if plugin_do_dispatch(handle, command, args) {
                println!("✅ Executed {} {}", command, args);

                // Return output string (for VariableMacro)
                Ok(Some(plugin_get_data(handle)))
            } else {
                eprintln!("❌ Failed {} {}", command, args);
                Err(Box::new(RunError::ErrorExecutingCommand))
            }
        }
    }

    pub fn run_script(
        &mut self,
        items: &mut Vec<Item>,
        plugin_manager: &mut PluginManager,
    ) -> Result<(), Box<dyn Error>> {
        let mut skiplabel = String::new();

        for item in items.iter_mut() {
            // Skip all tokens until we hit the right label
            if !skiplabel.is_empty() {
                if let TokenType::Label { label } = &item.token_type {
                    if *label == skiplabel {
                        println!("▶️ Found label '{}', resuming execution", label);
                        skiplabel.clear(); // stop skipping
                        continue;
                    }
                }
                // if we're still skipping, move to the next item
                if !skiplabel.is_empty() {
                    println!("🚫Skipping item: {:?}", item);
                    continue;
                }
            }

            match &mut item.token_type {
                TokenType::VariableMacro {
                    plugin,
                    command,
                    args,
                    vmacro,
                    ..
                } => {
                    let result = self
                        .execute_plugin_command(plugin_manager, plugin, command, args)?
                        .unwrap_or_default();
                    self.macros.insert(vmacro.clone(), result);
                }

                TokenType::Command {
                    plugin,
                    command,
                    args,
                    ..
                } => {
                    self.execute_plugin_command(plugin_manager, plugin, command, args)?;
                }

                TokenType::IfGoTo { condition, label } => {
                    string_replacer::replace_macros(condition, &self.macros);
                    if condition.is_empty() || condition.to_lowercase() == "true" {
                        println!("⏩ Skipping until label '{}'", label);
                        skiplabel = label.clone();
                        continue;
                    }
                }

                TokenType::Label { label } => {
                    // normal labels are handled in the skip block above
                    // (this will only run when not skipping)
                    println!("🏷️ Encountered label '{}'", label);
                }

                _ => {}
            }
        }

        Ok(())
    }
}
