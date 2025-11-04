use std::ffi::c_char;

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
    pub ptr: *mut std::ffi::c_void,
    pub do_dispatch: extern "C" fn(*mut std::ffi::c_void, *const c_char, *const c_char) -> bool,
    pub destroy: extern "C" fn(*mut std::ffi::c_void),
}

pub type PluginCreateFn = unsafe extern "C" fn() -> PluginHandle;
pub type PluginDestroyFn = extern "C" fn(*mut std::ffi::c_void);
pub type PluginPtr = *mut std::ffi::c_void;

pub type ParamsSet = std::collections::HashMap<String, String>;
pub type ParamsGet = std::collections::HashMap<String, String>;
