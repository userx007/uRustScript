use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::ffi::{CStr, CString};
use std::fmt;

use interfaces::{Item, TokenType, Validator};
use plugin_api::{ParamsGet, PARAMS_GET_CMDS_KEY};
use plugin_loader::load_plugins;

#[derive(Debug)]
enum ValidateError {
    PluginNotSetForLoading,
    PluginLoadingFailed,
}

impl fmt::Display for ValidateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ValidateError::PluginNotSetForLoading => {
                write!(f, "Needed plugins not plugins_to_load")
            }
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
        plugin_commands: &mut HashMap<String, HashSet<String>>,
    ) -> bool {
        let mut used: HashSet<String> = HashSet::new();

        for item in items {
            match &item.token_type {
                TokenType::LoadPlugin { plugin, .. } => {
                    plugins.insert(plugin.to_string());
                }
                TokenType::VariableMacro {
                    plugin, command, ..
                }
                | TokenType::Command {
                    plugin, command, ..
                } => {
                    used.insert(plugin.to_string());
                    plugin_commands
                        .entry(plugin.to_string())
                        .or_insert_with(HashSet::new)
                        .insert(command.to_string());
                }
                _ => {}
            }
        }

        println!("plugins_to_load: {:?}", plugins);
        println!("Used  : {:?}", used);

        if *plugins != used {
            let missing: HashSet<_> = used.difference(&plugins).cloned().collect();
            println!("Missing  : {:?}", missing);
            return false;
        }
        true
    }

    fn validate_plugins(&self, plugins: &HashSet<String>) -> bool {
        let plugins_to_load_plugins = load_plugins(plugins, "target/debug");

        println!("vPlugin:{}", plugins_to_load_plugins[0].name);

        let plugin = &plugins_to_load_plugins[0].handle;
        let cmd = CString::new("MECHO").unwrap();
        let args = CString::new("Hello from host").unwrap();

        (plugin.do_dispatch)(plugin.ptr, cmd.as_ptr(), args.as_ptr());

        unsafe {
            let c_str = (plugin.get_data)(plugin.ptr);
            let result = CStr::from_ptr(c_str).to_str().unwrap();
            println!("Result from plugin: {}", result);

            // (plugin.destroy)(plugin.ptr);
        }

        let mut params: ParamsGet = Default::default();
        (plugin.get_params)(plugin.ptr, &mut params);

        println!("params: {:?}", params);

        if let Some(cmds) = params.get(PARAMS_GET_CMDS_KEY) {
            println!("Commands list: {:?}", cmds);
        } else {
            println!("Not found..");
        }

        true
    }
}

impl Validator for ScriptValidator {
    fn validate_script(&self, items: &mut Vec<Item>) -> Result<(), Box<dyn Error>> {
        let mut plugins_to_load: HashSet<String> = HashSet::new();
        let mut plugins_commands: HashMap<String, HashSet<String>> = HashMap::new();

        println!("Validating script ...");
        if false
            == self.validate_plugins_availability(
                items,
                &mut plugins_to_load,
                &mut plugins_commands,
            )
        {
            return Err(Box::new(ValidateError::PluginNotSetForLoading));
        }

        println!("HM:{:?}", plugins_commands);

        if false == self.validate_plugins(&plugins_to_load) {
            return Err(Box::new(ValidateError::PluginLoadingFailed));
        }
        Ok(())
    }
}

/*

    fn validate_plugins(&self, plugins: &HashSet<String>) -> bool {
        let plugins_to_load_plugins = load_plugins(plugins, "target/debug");

        println!("vPlugin:{}", plugins_to_load_plugins[0].name);

        let plugin = &plugins_to_load_plugins[0].handle;
        let cmd = CString::new("ECHO").unwrap();
        let args = CString::new("Hello from host").unwrap();

        (plugin.do_dispatch)(plugin.ptr, cmd.as_ptr(), args.as_ptr());

        unsafe {
            let c_str = (plugin.get_data)(plugin.ptr);
            let result = CStr::from_ptr(c_str).to_str().unwrap();
            println!("Result from plugin: {}", result);

            // (plugin.destroy)(plugin.ptr);
        }

        let mut params: ParamsGet = Default::default();
        (plugin.get_params)(plugin.ptr, &mut params);

        println!("params: {:?}", params);

        if let Some(cmds) = params.get(PARAMS_GET_CMDS_KEY) {
            println!("Commands list: {:?}", cmds);
        } else {
            println!("Not found..");
        }

        true
    }

*/
