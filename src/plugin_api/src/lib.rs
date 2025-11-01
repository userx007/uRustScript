use std::os::raw::{c_char, c_void};

#[repr(C)]
pub struct PluginVTable {
    pub is_initialized: extern "C" fn(*const c_void) -> bool,
    pub is_enabled: extern "C" fn(*const c_void) -> bool,
    pub set_params: extern "C" fn(*mut c_void, *const c_void) -> bool,
    pub get_data: extern "C" fn(*const c_void) -> *const c_char,
    pub reset_data: extern "C" fn(*mut c_void),
    pub do_init: extern "C" fn(*mut c_void, *mut c_void) -> bool,
    pub do_enable: extern "C" fn(*mut c_void),
    pub do_dispatch: extern "C" fn(*mut c_void, *const c_char, *const c_char) -> bool,
    pub do_cleanup: extern "C" fn(*mut c_void),
}

#[repr(C)]
pub struct Plugin {
    pub instance: *mut c_void,
    pub vtable: *const PluginVTable,
}

pub type PluginCreateFn = extern "C" fn() -> Plugin;
pub type PluginDestroyFn = extern "C" fn(*mut Plugin);
