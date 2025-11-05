use std::collections::HashMap;
use std::ffi::{CString, CStr, c_char};

use plugin_api::{ParamsGet, ParamsSet, PluginHandle, PluginInterface, PluginPtr};
use plugin_macros::plugin_commands;


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
        println!("Called ECHO with args: {}", args);
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

extern "C" fn plugin_is_initialized(ptr: PluginPtr) -> bool {
    if ptr.is_null() {
        return false;
    }
    let plugin = unsafe { &*(ptr as *mut UtilsPlugin) };
    plugin.is_initialized()
}

extern "C" fn plugin_is_enabled(ptr: PluginPtr) -> bool {
    if ptr.is_null() {
        return false;
    }
    let plugin = unsafe { &*(ptr as *mut UtilsPlugin) };
    plugin.is_enabled()
}

extern "C" fn plugin_set_params(ptr: PluginPtr, params: *const ParamsSet) -> bool {
    if ptr.is_null() || params.is_null() {
        return false;
    }
    let plugin = unsafe { &mut *(ptr as *mut UtilsPlugin) };
    let params_ref = unsafe { &*params };
    plugin.set_params(params_ref)
}

extern "C" fn plugin_get_params(ptr: PluginPtr, params: *mut ParamsGet) {
    if ptr.is_null() || params.is_null() {
        return;
    }
    let plugin = unsafe { &*(ptr as *mut UtilsPlugin) };
    let params_ref = unsafe { &mut *params };
    plugin.get_params(params_ref);
}

extern "C" fn plugin_do_dispatch(ptr: PluginPtr, cmd: *const c_char, args: *const c_char) -> bool {
    if ptr.is_null() {
        return false;
    }
    let plugin = unsafe { &mut *(ptr as *mut UtilsPlugin) };
    let cmd_str = unsafe { CStr::from_ptr(cmd).to_str().unwrap_or_default() };
    let args_str = unsafe { CStr::from_ptr(args).to_str().unwrap_or_default() };
    plugin.do_dispatch(cmd_str, args_str)
}

extern "C" fn plugin_reset_data(ptr: PluginPtr) {
    if ptr.is_null() {
        return;
    }
    let plugin = unsafe { &mut *(ptr as *mut UtilsPlugin) };
    plugin.reset_data();
}

extern "C" fn plugin_get_data(ptr: PluginPtr) -> *const c_char {
    if ptr.is_null() {
        return std::ptr::null();
    }
    let plugin = unsafe { &*(ptr as *mut UtilsPlugin) };
    let s = plugin.get_data();
    CString::new(s).unwrap().into_raw()
}

extern "C" fn plugin_destroy(ptr: PluginPtr) {
    if !ptr.is_null() {
        unsafe { drop(Box::from_raw(ptr as *mut UtilsPlugin)) };
    }
}


#[no_mangle]
pub extern "C" fn plugin_create() -> PluginHandle {
    let plugin = Box::new(UtilsPlugin::new());

    PluginHandle {
        ptr: Box::into_raw(plugin) as PluginPtr,

        destroy: plugin_destroy,

        is_initialized: plugin_is_initialized,
        is_enabled: plugin_is_enabled,
        set_params: plugin_set_params,
        get_params: plugin_get_params,
        do_dispatch: plugin_do_dispatch,
        reset_data: plugin_reset_data,
        get_data: plugin_get_data,
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
