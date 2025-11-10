use interfaces::{Item, TokenType};
use std::error::Error;
use std::fmt;
use plugin_api::{PluginHandle, plugin_do_dispatch};
use plugin_manager::PluginManager;


#[derive(Debug)]
enum RunError {
    InvalidStatement,
}

impl fmt::Display for RunError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RunError::InvalidStatement => write!(f, "Invalid item in script"),
        }
    }
}

impl Error for RunError {}

pub struct ScriptRunner;

impl ScriptRunner {
    pub fn new() -> Self {
        ScriptRunner {}
    }

    pub fn run_script(&self, items: &mut Vec<Item>, plugin_manager: &mut PluginManager) -> Result<(), Box<dyn Error>> {
        println!("Running script ...");

/*
        for item in items {
            match &mut item.token_type {
                TokenType::VariableMacro { command, args, .. }
                | TokenType::Command { command, args, .. } => {

                    if !pluginptr.is_null() {
                        unsafe {
                            // cast back to PluginHandle
                            let plugin_handle = &mut *( *pluginptr as *mut PluginHandle );

                            // convert Rust strings to C pointers
                            let cmd_ptr = command.as_ptr() as *const i8;
                            let args_ptr = args.as_ptr() as *const i8;

                            if (plugin_handle.do_dispatch)(plugin_handle.ptr, cmd_ptr, args_ptr) {
                                println!("✅ Executed {} {}", command, args);
                            } else {
                                eprintln!("❌ Failed to execute {} {}", command, args);
                            }
                        }
                    }
                }
                _ => {}
            }
        }
*/
        Ok(())
    }
}
