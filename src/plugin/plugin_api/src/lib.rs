use std::collections::HashMap;
use std::ffi::{c_char, c_void, CStr, CString};

// ---------------------------
// Shared constants
// ---------------------------
pub const PARAMS_GET_CMDS_KEY: &str = "cmds";
pub const PARAMS_GET_VERS_KEY: &str = "vers";
pub const PARAMS_FAULT_TOLERANT: &str = "FAULT_TOLERANT";
pub const PARAMS_PRIVILEGED: &str = "PRIVILEGED";

// ---------------------------
// Shared type definitions
// ---------------------------
pub type ParamsSet = HashMap<String, String>;
pub type ParamsGet = HashMap<String, Vec<&'static str>>;
pub type PluginCreateFn = unsafe extern "C" fn() -> PluginHandle;

// ---------------------------
// Plugin trait
// ---------------------------
pub trait PluginInterface {
    fn do_init(&mut self, user_data: *mut c_void) -> bool;
    fn do_enable(&mut self);
    fn do_dispatch(&mut self, cmd: &str, args: &str) -> bool;
    fn do_cleanup(&mut self);
    fn set_params(&mut self, params: &ParamsSet) -> bool;
    fn get_params(&self, params: &mut ParamsGet);
    fn get_data(&self) -> &str;
    fn reset_data(&mut self);
    fn is_initialized(&self) -> bool;
    fn is_enabled(&self) -> bool;
    fn is_privileged(&self) -> bool;
    fn is_fault_tolerant(&self) -> bool;
}

// ---------------------------
// FFI-compatible handle
// ---------------------------
#[repr(C)]
pub struct PluginHandle {
    pub ptr: *mut c_void,
    pub destroy: unsafe extern "C" fn(*mut c_void),
    pub do_init: unsafe extern "C" fn(*mut c_void, *mut c_void) -> bool,
    pub do_enable: unsafe extern "C" fn(*mut c_void),
    pub do_dispatch: unsafe extern "C" fn(*mut c_void, *const c_char, *const c_char) -> bool,
    pub do_cleanup: unsafe extern "C" fn(*mut c_void),
    pub set_params: unsafe extern "C" fn(*mut c_void, *const ParamsSet) -> bool,
    pub get_params: unsafe extern "C" fn(*mut c_void, *mut ParamsGet),
    pub get_data: unsafe extern "C" fn(*mut c_void) -> *const c_char,
    pub reset_data: unsafe extern "C" fn(*mut c_void),
    pub is_initialized: unsafe extern "C" fn(*mut c_void) -> bool,
    pub is_enabled: unsafe extern "C" fn(*mut c_void) -> bool,
    pub is_privileged: unsafe extern "C" fn(*mut c_void) -> bool,
    pub is_fault_tolerant: unsafe extern "C" fn(*mut c_void) -> bool,
}

// ---------------------------
// Generic FFI handle builder
// ---------------------------
pub fn make_handle<T: PluginInterface + 'static>(plugin: T) -> PluginHandle {
    // --- Generic FFI glue ---
    unsafe extern "C" fn destroy<T: PluginInterface>(ptr: *mut c_void) {
        if !ptr.is_null() {
            drop(Box::from_raw(ptr.cast::<T>()));
        }
    }

    unsafe extern "C" fn do_init<T: PluginInterface>(ptr: *mut c_void, user_data: *mut c_void) -> bool {
        debug_assert!(!ptr.is_null());
        (&mut *ptr.cast::<T>()).do_init(user_data)
    }

    unsafe extern "C" fn do_enable<T: PluginInterface>(ptr: *mut c_void) {
        debug_assert!(!ptr.is_null());
        (&mut *ptr.cast::<T>()).do_enable();
    }

    unsafe extern "C" fn do_dispatch<T: PluginInterface>(
        ptr: *mut c_void,
        cmd: *const c_char,
        args: *const c_char,
    ) -> bool {
        debug_assert!(!ptr.is_null());
        let plugin = &mut *ptr.cast::<T>();
        let cmd_str = CStr::from_ptr(cmd).to_str().unwrap_or_default();
        let args_str = CStr::from_ptr(args).to_str().unwrap_or_default();
        plugin.do_dispatch(cmd_str, args_str)
    }

    unsafe extern "C" fn do_cleanup<T: PluginInterface>(ptr: *mut c_void) {
        debug_assert!(!ptr.is_null());
        (&mut *ptr.cast::<T>()).do_cleanup();
    }

    unsafe extern "C" fn set_params<T: PluginInterface>(
        ptr: *mut c_void,
        params: *const ParamsSet,
    ) -> bool {
        debug_assert!(!ptr.is_null());
        (&mut *ptr.cast::<T>()).set_params(&*params)
    }

    unsafe extern "C" fn get_params<T: PluginInterface>(ptr: *mut c_void, params: *mut ParamsGet) {
        debug_assert!(!ptr.is_null());
        (&*ptr.cast::<T>()).get_params(&mut *params);
    }

    unsafe extern "C" fn get_data<T: PluginInterface>(ptr: *mut c_void) -> *const c_char {
        debug_assert!(!ptr.is_null());
        let plugin = &*ptr.cast::<T>();
        CString::new(plugin.get_data())
            .unwrap_or_default()
            .into_raw()
    }

    unsafe extern "C" fn reset_data<T: PluginInterface>(ptr: *mut c_void) {
        debug_assert!(!ptr.is_null());
        (&mut *ptr.cast::<T>()).reset_data();
    }

    unsafe extern "C" fn is_initialized<T: PluginInterface>(ptr: *mut c_void) -> bool {
        debug_assert!(!ptr.is_null());
        (&*ptr.cast::<T>()).is_initialized()
    }

    unsafe extern "C" fn is_enabled<T: PluginInterface>(ptr: *mut c_void) -> bool {
        debug_assert!(!ptr.is_null());
        (&*ptr.cast::<T>()).is_enabled()
    }

    unsafe extern "C" fn is_privileged<T: PluginInterface>(ptr: *mut c_void) -> bool {
        debug_assert!(!ptr.is_null());
        (&*ptr.cast::<T>()).is_privileged()
    }

    unsafe extern "C" fn is_fault_tolerant<T: PluginInterface>(ptr: *mut c_void) -> bool {
        debug_assert!(!ptr.is_null());
        (&*ptr.cast::<T>()).is_fault_tolerant()
    }

    // --- Allocate and return handle ---
    let boxed = Box::new(plugin);

    PluginHandle {
        ptr: Box::into_raw(boxed).cast::<c_void>(),
        destroy: destroy::<T>,
        do_init: do_init::<T>,
        do_enable: do_enable::<T>,
        do_dispatch: do_dispatch::<T>,
        do_cleanup: do_cleanup::<T>,
        set_params: set_params::<T>,
        get_params: get_params::<T>,
        get_data: get_data::<T>,
        reset_data: reset_data::<T>,
        is_initialized: is_initialized::<T>,
        is_enabled: is_enabled::<T>,
        is_privileged: is_privileged::<T>,
        is_fault_tolerant: is_fault_tolerant::<T>,
    }
}

// ---------------------------
// Plugin function wrappers
// ---------------------------

/// # Safety
/// The caller must ensure `handle` points to a valid [`PluginHandle`].
#[allow(clippy::unnecessary_map_or)]
pub unsafe fn plugin_do_dispatch(handle: *mut PluginHandle, cmd: &str, args: &str) -> bool {
    handle.as_mut().map_or(false, |plugin| {
        let c_cmd = CString::new(cmd).expect("invalid cmd");
        let c_args = CString::new(args).expect("invalid args");

        let success = (plugin.do_dispatch)(plugin.ptr, c_cmd.as_ptr(), c_args.as_ptr());
        let is_fault_tolerant = (plugin.is_fault_tolerant)(plugin.ptr);

        success || is_fault_tolerant
    })
}

/// # Safety
/// The caller must ensure `handle` points to a valid [`PluginHandle`].
pub unsafe fn plugin_get_data(handle: *mut PluginHandle) -> String {
    handle.as_mut().map_or_else(String::new, |plugin| {
        let c_str = (plugin.get_data)(plugin.ptr);
        if c_str.is_null() {
            String::new()
        } else {
            CStr::from_ptr(c_str).to_string_lossy().into_owned()
        }
    })
}

/// # Safety
/// The caller must ensure `handle` points to a valid [`PluginHandle`].
pub unsafe fn plugin_do_enable(handle: *mut PluginHandle) -> bool {
    handle.as_mut().is_some_and(|plugin| {
        (plugin.do_enable)(plugin.ptr);
        true
    })
}
