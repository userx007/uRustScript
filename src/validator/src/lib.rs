use std::collections::HashSet;
use std::error::Error;
use std::fmt;

use interfaces::{Item, TokenType, Validator};
use plugin_loader::load_plugins;

use std::ffi::CString;

#[derive(Debug)]
enum ValidateError {
    PluginNotLoaded,
    PluginLoadingFailed,
}

impl fmt::Display for ValidateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ValidateError::PluginNotLoaded => write!(f, "Needed plugins not loaded"),
            ValidateError::PluginLoadingFailed => write!(f, "Failed to load plugin"),
        }
    }
}

impl Error for ValidateError {}

pub struct ScriptValidator;

impl ScriptValidator {
    pub fn new() -> Self {
        ScriptValidator {}
    }

    fn validate_plugins_availability(
        &self,
        items: &mut Vec<Item>,
        plugins: &mut HashSet<String>,
    ) -> bool {
        let mut used: HashSet<String> = HashSet::new();

        for item in items {
            match &item.token_type {
                TokenType::LoadPlugin { plugin, .. } => {
                    plugins.insert(plugin.to_string());
                }
                TokenType::VariableMacro { plugin, .. } => {
                    used.insert(plugin.to_string());
                }
                TokenType::Command { plugin, .. } => {
                    used.insert(plugin.to_string());
                }
                _ => {}
            }
        }

        println!("Loaded: {:?}", plugins);
        println!("Used  : {:?}", used);

        if *plugins != used {
            let missing: HashSet<_> = used.difference(&plugins).cloned().collect();
            println!("Missing  : {:?}", missing);
            return false;
        }
        true
    }

    fn validate_plugins(&self, plugins: &HashSet<String>) -> bool {
        let loaded_plugins = load_plugins(plugins, "target/debug");

        println!("vPlugin:{}", loaded_plugins[0].name);

        let handle = &loaded_plugins[0].handle;
        let cmd = CString::new("ECHO").unwrap();
        let args = CString::new("Hello from host").unwrap();

        (handle.do_dispatch)(handle.ptr, cmd.as_ptr(), args.as_ptr());

        true
    }
}

impl Validator for ScriptValidator {
    fn validate_script(&self, items: &mut Vec<Item>) -> Result<(), Box<dyn Error>> {
        let mut loaded = HashSet::<String>::new();

        println!("Validating script ...");
        if false == self.validate_plugins_availability(items, &mut loaded) {
            return Err(Box::new(ValidateError::PluginNotLoaded));
        }

        if false == self.validate_plugins(&loaded) {
            return Err(Box::new(ValidateError::PluginLoadingFailed));
        }
        Ok(())
    }
}
