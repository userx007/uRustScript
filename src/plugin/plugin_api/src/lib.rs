use std::ffi::{c_char, c_void};

pub trait PluginInterface {
    fn is_initialized(&self) -> bool;
    fn is_enabled(&self) -> bool;
    fn set_params(&mut self, params: &ParamsSet) -> bool;
    fn get_params(&self, params: &mut ParamsGet);
    fn do_dispatch(&mut self, cmd: &str, args: &str) -> bool;
    fn reset_data(&mut self);
    fn get_data(&self) -> &str;
}


#[repr(C)]
pub struct PluginHandle {
    pub ptr: *mut c_void,

    // Lifecycle and management
    pub destroy: extern "C" fn(*mut c_void),

    // Trait method mappings
    pub is_initialized: extern "C" fn(*mut c_void) -> bool,
    pub is_enabled: extern "C" fn(*mut c_void) -> bool,
    pub set_params: extern "C" fn(*mut c_void, *const ParamsSet) -> bool,
    pub get_params: extern "C" fn(*mut c_void, *mut ParamsGet),
    pub do_dispatch: extern "C" fn(*mut c_void, *const c_char, *const c_char) -> bool,
    pub reset_data: extern "C" fn(*mut c_void),
    pub get_data: extern "C" fn(*mut c_void) -> *const c_char,
}

pub type PluginPtr = *mut c_void;
pub type PluginCreateFn = unsafe extern "C" fn() -> PluginHandle;
pub type PluginDestroyFn = extern "C" fn(*mut std::ffi::c_void);
pub type ParamsSet = std::collections::HashMap<String, String>;
pub type ParamsGet = std::collections::HashMap<String, String>;
