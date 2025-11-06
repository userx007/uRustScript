use interfaces::{Item, Runner, TokenType};
use std::error::Error;
use std::fmt;

use plugin_api::{PluginHandle, plugin_do_dispatch};


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
}

impl Runner for ScriptRunner {
    fn run_script(&self, items: &mut Vec<Item>) -> Result<(), Box<dyn Error>> {
        println!("Running script ...");

        for item in items {
            match &mut item.token_type {
                TokenType::VariableMacro { command, args, pluginptr, .. }
                | TokenType::Command { command, args, pluginptr, .. } => {
                    unsafe {
                        if plugin_do_dispatch(*pluginptr as *mut PluginHandle, command, args) {
                            println!("✅ Executed {} {}", command, args);
                        } else {
                            eprintln!("❌ Failed to execute {} {}", command, args);
                        }
                    }
                }
                _ => {}
            }
        }

        Ok(())
    }
}
