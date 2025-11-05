use std::collections::{HashMap, HashSet};
use std::error::Error;
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

    fn validate_plugins_used_commands(
        &self,
        plugins: &HashSet<String>,
        plugin_used_commands: &mut HashMap<String, HashSet<String>>,
    ) -> bool {
        let plugins_to_load = load_plugins(plugins, "target/debug");

        for plugin in plugins_to_load {
            let plugin_handle = &plugin.handle;
            let mut params: ParamsGet = Default::default();
            (plugin_handle.get_params)(plugin_handle.ptr, &mut params);

            if let Some(plugin_supported_commands) = params.get(PARAMS_GET_CMDS_KEY) {
                println!(
                    "📝 Plugin {} -> Commands : {:?}",
                    plugin.name, plugin_supported_commands
                );

                // Find which commands are used in the script for this plugin
                let used_for_this_plugin = plugin_used_commands
                    .get(&plugin.name)
                    .cloned()
                    .unwrap_or_default();

                // Check which script commands are NOT supported by plugin
                let unsupported_used_commands: Vec<_> = used_for_this_plugin
                    .iter()
                    .filter(|cmd| !plugin_supported_commands.contains(&cmd.as_str()))
                    .collect();

                if !unsupported_used_commands.is_empty() {
                    println!(
                        "❌ Plugin '{}' missing script commands: {:?}",
                        plugin.name, unsupported_used_commands
                    );
                    return false;
                }
            } else {
                println!("❌ Section {:?} not found in ParamsGet", PARAMS_GET_CMDS_KEY);
                return false;
            }
        }
        println!("✅ Used commands supported by plugins");
        true
    }
}

impl Validator for ScriptValidator {
    fn validate_script(&self, items: &mut Vec<Item>) -> Result<(), Box<dyn Error>> {
        let mut plugins_to_load: HashSet<String> = HashSet::new();
        let mut plugin_used_commands: HashMap<String, HashSet<String>> = HashMap::new();

        println!("Validating script ...");
        if false
            == self.validate_plugins_availability(
                items,
                &mut plugins_to_load,
                &mut plugin_used_commands,
            )
        {
            return Err(Box::new(ValidateError::PluginNotSetForLoading));
        }

        println!("HM:{:?}", plugin_used_commands);

        if false == self.validate_plugins_used_commands(&plugins_to_load, &mut plugin_used_commands)
        {
            return Err(Box::new(ValidateError::PluginLoadingFailed));
        }
        Ok(())
    }
}

/*

    fn validate_plugins(&self, plugins: &HashSet<String>) -> bool {
        let plugins_to_load = load_plugins(plugins, "target/debug");

        println!("vPlugin:{}", plugins_to_load[0].name);

        let plugin = &plugins_to_load[0].handle;
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
