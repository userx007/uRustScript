use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fmt;

use interfaces::{Item, TokenType};
use plugin_api::{ParamsGet, PARAMS_GET_CMDS_KEY};
use plugin_manager::{PluginDescriptor, PluginManager};

#[derive(Debug)]
enum ValidateError {
    PluginNotSetForLoading,
    PluginLoadingFailed,
    PluginCommandAvailability,
    JumpsLabelMismatch,
}

impl fmt::Display for ValidateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
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
                        .or_default()
                        .insert(command.to_string());
                }
                _ => {}
            }
        }

        println!("used_plugins: {:?}", plugins);
        println!("Used  : {:?}", used);

        if *plugins != used {
            let missing: HashSet<_> = used.difference(plugins).cloned().collect();
            println!("Missing  : {:?}", missing);
            return false;
        }
        true
    }

    fn validate_plugins_commands(
        &self,
        plugin_commands: &mut HashMap<String, HashSet<String>>,
        plugin_manager: &mut PluginManager,
    ) -> bool {
        for (plugin_name, PluginDescriptor { handle, _lib }) in &plugin_manager.plugins {
            let mut params: ParamsGet = Default::default();

            unsafe {
                // `plugin_handle` is a reference to a *mut c_void
                //let handle_ptr = *handle as *mut PluginHandle;
                let handle_ptr = *handle;
                if !handle_ptr.is_null() {
                    ((*handle_ptr).get_params)((*handle_ptr).ptr, &mut params);
                }
            }

            if let Some(plugin_supported_commands) = params.get(PARAMS_GET_CMDS_KEY) {
                println!(
                    "📝 Plugin {} -> Commands : {:?}",
                    plugin_name, plugin_supported_commands
                );

                // Find which commands are used in the script for this plugin
                let used_for_this_plugin = plugin_commands
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

        println!("✅ Commands supported by plugins");
        true
    }

    fn validate_plugins_loading(
        &self,
        plugins: &HashSet<String>,
        plugin_manager: &mut PluginManager,
    ) -> bool {
        plugin_manager.load_plugins(plugins)
    }

    fn validate_jumps(&self, items: &[Item]) -> bool {
        let mut pending_jumps: HashMap<String, usize> = HashMap::new();
        let mut defined_labels: HashSet<String> = HashSet::new();

        for item in items.iter() {
            match &item.token_type {
                TokenType::IfGoTo { label, .. } => {
                    // Increment pending jump for this label
                    *pending_jumps.entry(label.clone()).or_insert(0) += 1;
                }

                TokenType::Label { label } => {
                    if let Some(count) = pending_jumps.get_mut(label) {
                        if *count > 0 {
                            // Jump exists → label is valid
                            *count -= 1;
                            defined_labels.insert(label.clone());
                        } else {
                            // No jump pending → invalid label
                            eprintln!("❌ Invalid label '{}' without preceding jump", label);
                            return false;
                        }
                    } else {
                        // Label appeared without any jump
                        eprintln!("❌ Label '{}' without any jump", label);
                        return false;
                    }
                }

                _ => {}
            }
        }

        // Check if any jump left without a label
        for (label, count) in pending_jumps.iter() {
            if *count > 0 {
                eprintln!("❌ Jump(s) to '{}' without corresponding label", label);
                return false;
            }
        }

        true
    }

    pub fn validate_script(
        &self,
        items: &mut Vec<Item>,
        plugin_manager: &mut PluginManager,
    ) -> Result<(), Box<dyn Error>> {
        let mut used_plugins: HashSet<String> = HashSet::new();
        let mut plugin_commands: HashMap<String, HashSet<String>> = HashMap::new();

        println!("Validating script ...");

        if !self.validate_jumps(items) {
            return Err(Box::new(ValidateError::JumpsLabelMismatch));
        }

        if !self.validate_plugins_availability(items, &mut used_plugins, &mut plugin_commands) {
            return Err(Box::new(ValidateError::PluginNotSetForLoading));
        }

        if !self.validate_plugins_loading(&used_plugins, plugin_manager) {
            return Err(Box::new(ValidateError::PluginLoadingFailed));
        }

        if !self.validate_plugins_commands(&mut plugin_commands, plugin_manager) {
            return Err(Box::new(ValidateError::PluginCommandAvailability));
        }

        Ok(())
    }
}

impl Default for ScriptValidator {
    fn default() -> Self {
        Self::new()
    }
}
