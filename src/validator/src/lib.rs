use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fmt;

use interfaces::{Item, TokenType, Validator};
use plugin_api::{ParamsGet, PluginIdentifier, PARAMS_GET_CMDS_KEY};
use plugin_loader::load_plugins;

#[derive(Debug)]
enum ValidateError {
    PluginNotSetForLoading,
    PluginLoadingFailed,
    PluginCommandAvailability,
}

impl fmt::Display for ValidateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ValidateError::PluginNotSetForLoading => {
                write!(f, "Needed plugins not plugins_to_load")
            }
            ValidateError::PluginLoadingFailed => write!(f, "Failed to load plugin"),
            ValidateError::PluginCommandAvailability => {
                write!(f, "Command not supported by plugin")
            }
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

    fn validate_plugins_loading(
        &self,
        plugins: &HashSet<String>,
        plugins_loaded: &mut PluginIdentifier,
    ) -> bool {
        *plugins_loaded = load_plugins(plugins, "target/debug");
        true
    }

    fn validate_plugins_commands(
        &self,
        plugins_loaded: &PluginIdentifier,
        plugin_used_commands: &mut HashMap<String, HashSet<String>>,
    ) -> bool {
        for (plugin_name, (plugin_handle, _)) in plugins_loaded {
            let mut params: ParamsGet = Default::default();

            unsafe {
                if plugin_handle.is_null() {
                    eprintln!("❌ Plugin '{}' handle is null", plugin_name);
                    return false;
                }

                // `plugin_handle` is &*mut PluginHandle, so deref twice:
                let plugin = &mut **plugin_handle; // &mut PluginHandle
                (plugin.get_params)(plugin.ptr, &mut params);
            }

            if let Some(plugin_supported_commands) = params.get(PARAMS_GET_CMDS_KEY) {
                println!(
                    "📝 Plugin {} -> Commands : {:?}",
                    plugin_name, plugin_supported_commands
                );

                // Find which commands are used in the script for this plugin
                let used_for_this_plugin = plugin_used_commands
                    .get(plugin_name)
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
                        plugin_name, unsupported_used_commands
                    );
                    return false;
                }
            } else {
                println!(
                    "❌ Section {:?} not found in ParamsGet",
                    PARAMS_GET_CMDS_KEY
                );
                return false;
            }
        }

        println!("✅ Used commands supported by plugins");
        true
    }

    fn insert_plugin_pointers(&self, items: &mut Vec<Item>, plugins: &PluginIdentifier) -> bool {
        for item in items {
            if let TokenType::VariableMacro {
                plugin, pluginptr, ..
            }
            | TokenType::Command {
                plugin, pluginptr, ..
            } = &mut item.token_type
            {
                if let Some((got_plugin_ptr, _)) = plugins.get(plugin) {
                    unsafe {
                        if got_plugin_ptr.is_null() {
                            eprintln!("⚠️ Plugin '{}' handle is null", plugin);
                            continue;
                        }

                        // Deref twice to get the PluginHandle structure
                        let handle = &mut **got_plugin_ptr;

                        if !handle.ptr.is_null() {
                            *pluginptr = handle.ptr;
                            println!("🔗 Set pluginptr for '{}' to {:?}", plugin, *pluginptr);
                        } else {
                            eprintln!("⚠️ Plugin '{}' internal ptr is null", plugin);
                        }
                    }
                } else {
                    eprintln!("⚠️ Plugin '{}' not found in loaded plugins", plugin);
                }
            }
        }
        true
    }
}

impl Validator for ScriptValidator {
    fn validate_script(&self, items: &mut Vec<Item>) -> Result<(), Box<dyn Error>> {
        let mut plugins_to_load: HashSet<String> = HashSet::new();
        let mut plugins_loaded: PluginIdentifier = HashMap::new();
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

        if false == self.validate_plugins_loading(&plugins_to_load, &mut plugins_loaded) {
            return Err(Box::new(ValidateError::PluginLoadingFailed));
        }

        if false == self.validate_plugins_commands(&plugins_loaded, &mut plugin_used_commands) {
            return Err(Box::new(ValidateError::PluginCommandAvailability));
        }

        if false == self.insert_plugin_pointers(items, &plugins_loaded) {
            return Err(Box::new(ValidateError::PluginCommandAvailability));
        }

        Ok(())
    }
}
