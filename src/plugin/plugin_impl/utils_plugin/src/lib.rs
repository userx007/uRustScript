use plugin_api::{
    make_handle, ParamsGet, ParamsSet, PluginHandle, PluginInterface, PARAMS_GET_CMDS_KEY,
};
use plugin_macros::plugin_commands;
use std::collections::HashMap;

pub struct UtilsPlugin {
    initialized: bool,
    enabled: bool,
    privileged: bool,
    fault_tolerant: bool,
    result: String,
    commands: HashMap<String, Box<dyn Fn(&mut Self, &str) -> bool>>,
    params_get: ParamsGet,
    params_set: ParamsSet,
}

impl UtilsPlugin {
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
        plugin
            .params_get
            .insert(PARAMS_GET_CMDS_KEY.to_string(), plugin.command_names());
        plugin
    }
}

// ---------------------- Commands ----------------------

#[allow(non_snake_case)]
#[plugin_commands]
impl UtilsPlugin {
    fn UECHO(&mut self, args: &str) -> bool {
        println!("Called ECHO with args: {}", args);
        self.result = args.to_string();
        true
    }

    fn URESET(&mut self, _args: &str) -> bool {
        self.result.clear();
        true
    }

    fn UPRINT(&mut self, args: &str) -> bool {
        println!("Plugin PRINT: {}", args);
        true
    }

    // Add more commands here as needed
}

// ---------------------- PluginInterface ----------------------

impl PluginInterface for UtilsPlugin {
    fn is_initialized(&self) -> bool {
        self.initialized
    }
    fn is_enabled(&self) -> bool {
        self.enabled
    }
    fn set_params(&mut self, params: &ParamsSet) -> bool {
        println!("set_params:");
        for (k, v) in params {
            println!("  {} = {}", k, v);
        }
        self.initialized = true;
        true
    }
    fn get_params(&self, params: &mut ParamsGet) {
        *params = self.params_get.clone();
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
    make_handle(UtilsPlugin::new())
}
