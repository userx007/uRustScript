use std::collections::HashMap;
use std::ffi::{c_char, c_void, CStr, CString};

// ---------------------------
// Shared type definitions
// ---------------------------

pub type ParamsSet = HashMap<String, String>;
pub type ParamsGet = HashMap<String, String>;

// ---------------------------
// Plugin trait
// ---------------------------

pub trait PluginInterface {
    fn is_initialized(&self) -> bool;
    fn is_enabled(&self) -> bool;
    fn set_params(&mut self, params: &ParamsSet) -> bool;
    fn get_params(&self, params: &mut ParamsGet);
    fn do_dispatch(&mut self, cmd: &str, args: &str) -> bool;
    fn reset_data(&mut self);
    fn get_data(&self) -> &str;
}

// ---------------------------
// FFI-compatible handle
// ---------------------------

#[repr(C)]
pub struct PluginHandle {
    pub ptr: *mut c_void,
    pub destroy: extern "C" fn(*mut c_void),
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

// ---------------------------
// Generic FFI handle builder
// ---------------------------

pub fn make_handle<T: PluginInterface + 'static>(plugin: T) -> PluginHandle {
    extern "C" fn destroy<T: PluginInterface>(ptr: *mut c_void) {
        if !ptr.is_null() {
            unsafe {
                drop(Box::from_raw(ptr as *mut T));
            }
        }
    }

    extern "C" fn is_initialized<T: PluginInterface>(ptr: *mut c_void) -> bool {
        unsafe { &*(ptr as *mut T) }.is_initialized()
    }

    extern "C" fn is_enabled<T: PluginInterface>(ptr: *mut c_void) -> bool {
        unsafe { &*(ptr as *mut T) }.is_enabled()
    }

    extern "C" fn set_params<T: PluginInterface>(
        ptr: *mut c_void,
        params: *const ParamsSet,
    ) -> bool {
        unsafe { &mut *(ptr as *mut T) }.set_params(unsafe { &*params })
    }

    extern "C" fn get_params<T: PluginInterface>(ptr: *mut c_void, params: *mut ParamsGet) {
        unsafe { &*(ptr as *mut T) }.get_params(unsafe { &mut *params });
    }

    extern "C" fn do_dispatch<T: PluginInterface>(
        ptr: *mut c_void,
        cmd: *const c_char,
        args: *const c_char,
    ) -> bool {
        let plugin = unsafe { &mut *(ptr as *mut T) };
        let cmd_str = unsafe { CStr::from_ptr(cmd).to_str().unwrap_or_default() };
        let args_str = unsafe { CStr::from_ptr(args).to_str().unwrap_or_default() };
        plugin.do_dispatch(cmd_str, args_str)
    }

    extern "C" fn reset_data<T: PluginInterface>(ptr: *mut c_void) {
        unsafe { &mut *(ptr as *mut T) }.reset_data();
    }

    extern "C" fn get_data<T: PluginInterface>(ptr: *mut c_void) -> *const c_char {
        let plugin = unsafe { &*(ptr as *mut T) };
        CString::new(plugin.get_data()).unwrap().into_raw()
    }

    let boxed = Box::new(plugin);

    PluginHandle {
        ptr: Box::into_raw(boxed) as *mut c_void,
        destroy: destroy::<T>,
        is_initialized: is_initialized::<T>,
        is_enabled: is_enabled::<T>,
        set_params: set_params::<T>,
        get_params: get_params::<T>,
        do_dispatch: do_dispatch::<T>,
        reset_data: reset_data::<T>,
        get_data: get_data::<T>,
    }
}
