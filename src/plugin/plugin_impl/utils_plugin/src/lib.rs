use plugin_api::*;
use plugin_macros::plugin_commands;
use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

// ---------------------- FFI Types ----------------------

/// Opaque pointer type for FFI
pub type PluginPtr = *mut std::ffi::c_void;

/// Function pointer types for FFI
pub type DispatchFn = extern "C" fn(PluginPtr, *const c_char, *const c_char) -> bool;
pub type DestroyFn = extern "C" fn(PluginPtr);

#[repr(C)]
pub struct PluginHandle {
    pub ptr: PluginPtr,
    pub do_dispatch: DispatchFn,
    pub destroy: DestroyFn,
}

// ---------------------- Plugin Struct ----------------------

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
        plugin.register_commands(); // procedural macro populates commands
        plugin
    }
}

// ---------------------- Commands ----------------------

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

    // Add more commands here as needed
}

// ---------------------- PluginInterface ----------------------

impl PluginInterface for UtilsPlugin {
    fn is_initialized(&self) -> bool { self.initialized }
    fn is_enabled(&self) -> bool { self.enabled }
    fn set_params(&mut self, _params: &ParamsSet) -> bool { self.initialized = true; true }
    fn get_params(&self, _params: &mut ParamsGet) {}
    fn reset_data(&mut self) { self.result.clear() }
    fn get_data(&self) -> &str { &self.result }

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
}

// ---------------------- FFI Wrappers ----------------------

extern "C" fn utils_dispatch(ptr: PluginPtr, cmd: *const c_char, args: *const c_char) -> bool {
    if ptr.is_null() { return false; }

    let plugin = unsafe { &mut *(ptr as *mut UtilsPlugin) };

    let cmd_str = unsafe { CStr::from_ptr(cmd).to_str().unwrap_or_default() };
    let args_str = unsafe { CStr::from_ptr(args).to_str().unwrap_or_default() };

    plugin.do_dispatch(cmd_str, args_str)
}

extern "C" fn utils_destroy(ptr: PluginPtr) {
    if !ptr.is_null() {
        unsafe { drop(Box::from_raw(ptr as *mut UtilsPlugin)); }
    }
}

#[no_mangle]
pub extern "C" fn plugin_create() -> PluginHandle {
    let plugin = Box::new(UtilsPlugin::new());
    PluginHandle {
        ptr: Box::into_raw(plugin) as PluginPtr,
        do_dispatch: utils_dispatch,
        destroy: utils_destroy,
    }
}

/* ---------------------- Example host usage ----------------------

use std::ffi::CString;
use libloading::Library;

unsafe {
    let lib = Library::new("utils_plugin.so").unwrap();

    let plugin_create: libloading::Symbol<unsafe extern "C" fn() -> PluginHandle> =
        lib.get(b"plugin_create").unwrap();

    let plugin = plugin_create();

    let cmd = CString::new("ECHO").unwrap();
    let args = CString::new("Hello from host").unwrap();

    (plugin.do_dispatch)(plugin.ptr, cmd.as_ptr(), args.as_ptr());

    // Optionally read internal data
    let plugin_obj = &mut *(plugin.ptr as *mut UtilsPlugin);
    println!("Result: {}", plugin_obj.get_data());

    (plugin.destroy)(plugin.ptr);
}

---------------------------------------------------------------------- */
