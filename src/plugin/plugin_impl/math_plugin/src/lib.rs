use plugin_api::{
    make_handle, ParamsGet, ParamsSet, PluginHandle, PluginInterface, PARAMS_FAULT_TOLERANT,
    PARAMS_GET_CMDS_KEY, PARAMS_GET_VERS_KEY, PARAMS_PRIVILEGED,
};
use plugin_macros::plugin_commands;
use std::collections::HashMap;
use utils::string_utils;

const PLUGIN_VERS: &str = "1.0.0.0";
type CommandFn<T> = Box<dyn Fn(&mut T, &str) -> bool>;

pub struct MathPlugin {
    initialized: bool,
    enabled: bool,
    privileged: bool,
    fault_tolerant: bool,
    result: String,
    commands: HashMap<String, CommandFn<Self>>,
    params_get: ParamsGet,
    params_set: ParamsSet,
}

impl MathPlugin {
    pub fn new() -> Self {
        let mut plugin = Self {
            initialized: false,
            enabled: false,
            privileged: false,
            fault_tolerant: false,
            result: String::new(),
            commands: HashMap::new(),
            params_get: HashMap::new(),
            params_set: HashMap::new(),
        };

        plugin.register_commands(); // procedural macro populates commands
        plugin.params_get.extend([
            (PARAMS_GET_CMDS_KEY.to_string(), plugin.command_names()),
            (PARAMS_GET_VERS_KEY.to_string(), vec![PLUGIN_VERS]),
        ]);
        plugin
    }
}

// ---------------------- Commands ----------------------

#[allow(non_snake_case)]
#[plugin_commands]
impl MathPlugin {
    fn MECHO(&mut self, args: &str) -> bool {
        if !self.is_enabled() {
            println!("NOT_ENABLED::Called MECHO with args: {}", args);
        } else {
            println!("ENABLED::Called MECHO with args: {}", args);
        }

        self.result = args.to_string();
        true
    }

    fn MRESET(&mut self, _args: &str) -> bool {
        self.result.clear();
        true
    }

    fn MPRINT(&mut self, args: &str) -> bool {
        println!("Plugin PRINT: {}", args);
        true
    }

    // Add more commands here as needed
}

// ---------------------- PluginInterface ----------------------

impl PluginInterface for MathPlugin {
    fn is_initialized(&self) -> bool {
        self.initialized
    }
    fn is_enabled(&self) -> bool {
        self.enabled
    }
    fn set_params(&mut self, params: &ParamsSet) -> bool {
        if let Some(fault_tolerant) = params.get(PARAMS_FAULT_TOLERANT) {
            if !string_utils::string_to_bool(fault_tolerant, &mut self.fault_tolerant) {
                println!(
                    "Invalid value for: {} -> {}",
                    PARAMS_FAULT_TOLERANT, fault_tolerant
                );
                return false;
            }
        }
        if let Some(privileged) = params.get(PARAMS_PRIVILEGED) {
            if !string_utils::string_to_bool(privileged, &mut self.privileged) {
                println!("Invalid value for: {} -> {}", PARAMS_PRIVILEGED, privileged);
                return false;
            }
        }
        self.initialized = true;
        true
    }
    fn get_params(&self, params: &mut ParamsGet) {
        *params = self.params_get.clone();
    }
    fn do_enable(&mut self) {
        self.enabled = true
    }
    fn reset_data(&mut self) {
        self.result.clear()
    }
    fn get_data(&self) -> &str {
        &self.result
    }
    fn do_dispatch(&mut self, cmd: &str, args: &str) -> bool {
        // avoid mutable/immutable borrow conflict
        if let Some(f) = self.commands.remove(cmd) {
            let result = f(self, args);
            self.commands.insert(cmd.to_string(), f); // put closure back
            result
        } else {
            false
        }
    }
    fn is_fault_tolerant(&self) -> bool {
        self.fault_tolerant
    }
    fn is_privileged(&self) -> bool {
        self.privileged
    }
}

#[no_mangle]
pub extern "C" fn plugin_create() -> PluginHandle {
    make_handle(MathPlugin::new())
}

impl Default for MathPlugin {
    fn default() -> Self {
        Self::new()
    }
}
