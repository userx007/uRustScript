use plugin_api::{
    make_handle, ParamsGet, ParamsSet, PluginHandle, PluginInterface, PARAMS_FAULT_TOLERANT,
    PARAMS_GET_CMDS_KEY, PARAMS_GET_VERS_KEY, PARAMS_PRIVILEGED,
};
use plugin_macros::plugin_commands;
use std::collections::HashMap;
use utils::string_utils;

const PLUGIN_VERS: &str = "1.0.0.0";
type CommandFn<T> = Box<dyn Fn(&mut T, &str) -> bool>;

pub struct UtilsPlugin {
    initialized: bool,
    enabled: bool,
    privileged: bool,
    fault_tolerant: bool,
    result: String,
    commands: HashMap<String, CommandFn<Self>>,
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

        plugin.register_commands();
        plugin.params_get.extend([
            (PARAMS_GET_CMDS_KEY.to_string(), plugin.command_names()),
            (PARAMS_GET_VERS_KEY.to_string(), vec![PLUGIN_VERS]),
        ]);
        plugin
    }
}

#[allow(non_snake_case)]
#[plugin_commands]
impl UtilsPlugin {
    fn UECHO(&mut self, args: &str) -> bool {
        if !self.is_enabled() {
            println!("NOT_ENABLED::Called UECHO with args: {}", args);
        } else {
            println!("ENABLED::Called UECHO with args: {}", args);
        }
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
}

impl PluginInterface for UtilsPlugin {
    fn do_init(&mut self) {
        self.initialized = true;
    }
    fn do_enable(&mut self) {
        self.enabled = true;
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
    fn do_cleanup(&mut self) {
        unimplemented!()
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
        true
    }
    fn get_params(&self, params: &mut ParamsGet) {
        *params = self.params_get.clone();
    }
    fn get_data(&self) -> &str {
        &self.result
    }
    fn reset_data(&mut self) {
        self.result.clear()
    }
    fn is_initialized(&self) -> bool {
        self.initialized
    }
    fn is_enabled(&self) -> bool {
        self.enabled
    }
    fn is_privileged(&self) -> bool {
        self.privileged
    }
    fn is_fault_tolerant(&self) -> bool {
        self.fault_tolerant
    }
}

impl Default for UtilsPlugin {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------
// C++ compatible entry/exit
// ---------------------------

#[no_mangle]
pub extern "C" fn pluginEntry() -> *mut PluginHandle {
    // Allocate the plugin handle on the heap
    let handle = make_handle(UtilsPlugin::new());
    Box::into_raw(Box::new(handle))
}

#[no_mangle]
pub extern "C" fn pluginExit(ptr_plugin: *mut PluginHandle) {
    if !ptr_plugin.is_null() {
        unsafe {
            // Take ownership of the handle
            let handle = Box::from_raw(ptr_plugin);
            // Call destroy on the inner plugin
            (handle.destroy)(handle.ptr);
            // handle itself is dropped here
        }
    }
}
