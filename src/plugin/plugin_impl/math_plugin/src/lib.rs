use plugin_interface::*;
use std::ffi::CString;
use std::ffi::c_char;

struct MathPlugin {
    initialized: bool,
    enabled: bool,
    data: String,
}

impl MathPlugin {
    extern "C" fn is_initialized(ptr: *const std::ffi::c_void) -> bool {
        unsafe { (*(ptr as *const Self)).initialized }
    }

    extern "C" fn is_enabled(ptr: *const std::ffi::c_void) -> bool {
        unsafe { (*(ptr as *const Self)).enabled }
    }

    extern "C" fn set_params(_ptr: *mut std::ffi::c_void, _params: *const std::ffi::c_void) -> bool {
        true
    }

    extern "C" fn get_data(ptr: *const std::ffi::c_void) -> *const std::os::raw::c_char {
        let plugin = unsafe { &*(ptr as *const Self) };
        CString::new(plugin.data.clone()).unwrap().into_raw()
    }

    extern "C" fn reset_data(ptr: *mut std::ffi::c_void) {
        unsafe { (*(ptr as *mut Self)).data.clear(); }
    }

    extern "C" fn do_init(ptr: *mut std::ffi::c_void, _user: *mut std::ffi::c_void) -> bool {
        unsafe { (*(ptr as *mut Self)).initialized = true; }
        true
    }

    extern "C" fn do_enable(ptr: *mut std::ffi::c_void) {
        unsafe { (*(ptr as *mut Self)).enabled = true; }
    }

    extern "C" fn do_dispatch(ptr: *mut std::ffi::c_void, cmd: *const c_char, params: *const c_char) -> bool {
        let cmd = unsafe { std::ffi::CStr::from_ptr(cmd) }.to_string_lossy();
        let params = unsafe { std::ffi::CStr::from_ptr(params) }.to_string_lossy();
        let plugin = unsafe { &mut *(ptr as *mut Self) };
        plugin.data = format!("Executed {cmd}({params})");
        true
    }

    extern "C" fn do_cleanup(ptr: *mut std::ffi::c_void) {
        unsafe { (*(ptr as *mut Self)).initialized = false; }
    }
}

static VTABLE: PluginVTable = PluginVTable {
    is_initialized: MathPlugin::is_initialized,
    is_enabled: MathPlugin::is_enabled,
    set_params: MathPlugin::set_params,
    get_data: MathPlugin::get_data,
    reset_data: MathPlugin::reset_data,
    do_init: MathPlugin::do_init,
    do_enable: MathPlugin::do_enable,
    do_dispatch: MathPlugin::do_dispatch,
    do_cleanup: MathPlugin::do_cleanup,
};

#[no_mangle]
pub extern "C" fn create_plugin() -> Plugin {
    let boxed = Box::new(MathPlugin {
        initialized: false,
        enabled: false,
        data: String::new(),
    });
    Plugin {
        instance: Box::into_raw(boxed) as *mut _,
        vtable: &VTABLE,
    }
}

#[no_mangle]
pub extern "C" fn destroy_plugin(p: *mut Plugin) {
    if p.is_null() {
        return;
    }
    unsafe {
        drop(Box::from_raw((*p).instance as *mut MathPlugin));
    }
}
