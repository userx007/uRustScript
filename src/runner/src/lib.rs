use interfaces::{Item, TokenType};
use plugin_api::{plugin_do_dispatch, plugin_get_data, PluginHandle};
use plugin_manager::PluginManager;
use std::error::Error;
use std::fmt;

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

pub struct ScriptRunner;

impl ScriptRunner {
    pub fn new() -> Self {
        ScriptRunner {}
    }

    fn execute_plugin_command(
        &self,
        plugin_manager: &mut PluginManager,
        plugin: &str,
        command: &str,
        args: &str,
    ) -> Result<Option<String>, Box<dyn Error>> {
        let descriptor = plugin_manager
            .plugins
            .get(plugin)
            .ok_or_else(|| RunError::PluginNotFound)?;

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
        &self,
        items: &mut Vec<Item>,
        plugin_manager: &mut PluginManager,
    ) -> Result<(), Box<dyn Error>> {
        for item in items {
            match &mut item.token_type {
                TokenType::VariableMacro {
                    plugin,
                    command,
                    args,
                    value,
                    ..
                } => {
                    let result =
                        self.execute_plugin_command(plugin_manager, plugin, command, args)?;
                    *value = result.unwrap_or_default();
                }

                TokenType::Command {
                    plugin,
                    command,
                    args,
                    ..
                } => {
                    self.execute_plugin_command(plugin_manager, plugin, command, args)?;
                }

                _ => {}
            }
        }

        Ok(())
    }
}
