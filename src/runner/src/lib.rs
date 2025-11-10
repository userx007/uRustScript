use interfaces::{Item, TokenType};
use plugin_api::{plugin_do_dispatch, PluginHandle};
use plugin_manager::PluginManager;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
enum RunError {
    ErrorExecutingCommand,
}

impl fmt::Display for RunError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RunError::ErrorExecutingCommand => write!(f, "Error executing command"),
        }
    }
}

impl Error for RunError {}

pub struct ScriptRunner;

impl ScriptRunner {
    pub fn new() -> Self {
        ScriptRunner {}
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
                    ..
                }
                | TokenType::Command {
                    plugin,
                    command,
                    args,
                    ..
                } => {
                    if let Some(descriptor) = plugin_manager.plugins.get(plugin) {
                        unsafe {
                            let handle: &mut PluginHandle = &mut *descriptor.handle;
                            if plugin_do_dispatch(handle, command, args) {
                                println!("✅ Executed {} {}", command, args);
                            } else {
                                eprintln!("❌ Failed {} {}", command, args);
                                return Err(Box::new(RunError::ErrorExecutingCommand));
                            }
                        }
                    }
                }
                _ => {}
            }
        }
        Ok(())
    }
}
