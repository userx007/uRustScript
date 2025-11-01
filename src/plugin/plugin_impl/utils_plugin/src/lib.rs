use plugin_api::*;
use plugin_macros::plugin_commands;
use std::collections::HashMap;

pub struct UtilsPlugin {
    initialized: bool,
    enabled: bool,
    result: String,
    commands: HashMap<String, Box<dyn Fn(&mut Self, &str) -> bool>>,
}

impl UtilsPlugin {
    pub fn new() -> Self {
        let mut plugin = Self {
            initialized: false,
            enabled: false,
            result: String::new(),
            commands: HashMap::new(),
        };
        plugin.register_commands(); // procedural macro registers all commands
        plugin
    }
}

#[plugin_commands]
impl UtilsPlugin {
    fn ECHO(&mut self, args: &str) -> bool {
        self.result = args.to_string();
        true
    }

    fn RESET(&mut self, _args: &str) -> bool {
        self.result.clear();
        true
    }

    fn PRINT(&mut self, args: &str) -> bool {
        println!("Plugin PRINT: {}", args);
        true
    }

    // Add any additional commands here
}

impl PluginInterface for UtilsPlugin {
    fn is_initialized(&self) -> bool {
        self.initialized
    }

    fn is_enabled(&self) -> bool {
        self.enabled
    }

    fn set_params(&mut self, _params: &ParamsSet) -> bool {
        self.initialized = true;
        true
    }

    fn get_params(&self, _params: &mut ParamsGet) {}

    fn reset_data(&mut self) {
        self.result.clear()
    }

    fn get_data(&self) -> &str {
        &self.result
    }

    fn do_dispatch(&mut self, cmd: &str, args: &str) -> bool {
        // Temporary removal to avoid mutable/immutable borrow conflict
        if let Some(f) = self.commands.remove(cmd) {
            let result = f(self, args);
            self.commands.insert(cmd.to_string(), f); // put closure back
            result
        } else {
            false
        }
    }
}

#[no_mangle]
pub extern "C" fn plugin_create() -> *mut dyn PluginInterface {
    let plugin = Box::new(UtilsPlugin::new());
    Box::into_raw(plugin)
}

#[no_mangle]
pub extern "C" fn plugin_destroy(plugin: *mut dyn PluginInterface) {
    if !plugin.is_null() {
        unsafe { Box::from_raw(plugin); }
    }
}


/*

use libloading::{Library, Symbol};
use plugin_api::PluginInterface;

unsafe {
    let lib = Library::new("utils_plugin.so").unwrap();

    let plugin_create: Symbol<unsafe extern "C" fn() -> *mut dyn PluginInterface> =
        lib.get(b"plugin_create").unwrap();
    let plugin_destroy: Symbol<unsafe extern "C" fn(*mut dyn PluginInterface)> =
        lib.get(b"plugin_destroy").unwrap();

    let plugin_ptr = plugin_create();
    let plugin = &mut *plugin_ptr;

    plugin.do_dispatch("ECHO", "Hello from dynamic load");
    println!("Result: {}", plugin.get_data());

    plugin_destroy(plugin_ptr);
}


*/