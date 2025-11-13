use libloading::{Library, Symbol};
use std::collections::{HashMap, HashSet};
use std::path::Path;

use plugin_api::{plugin_do_enable, PluginCreateFn, PluginHandle};
use utils::ini_parser::IniParserEx;

#[cfg(target_os = "windows")]
const LIB_EXT: &str = "dll";
#[cfg(target_os = "linux")]
const LIB_EXT: &str = "so";
#[cfg(target_os = "macos")]
const LIB_EXT: &str = "dylib";

const INI_SEARCH_DEPTH: usize = 5;

pub struct PluginDescriptor {
    pub handle: *mut PluginHandle,
    pub _lib: Library, // underscore means “used to hold lifetime”
    pub exit_fn: Option<unsafe extern "C" fn(*mut PluginHandle)>,
}

pub struct PluginManager {
    pluginsdirpath: &'static str,
    inipathname: &'static str,
    iniparser: IniParserEx,
    pub plugins: HashMap<String, PluginDescriptor>,
}

impl PluginManager {
    pub fn new(pluginsdirpath: &'static str, inipathname: &'static str) -> Self {
        Self {
            pluginsdirpath,
            inipathname,
            iniparser: IniParserEx::default(),
            plugins: HashMap::new(),
        }
    }

    pub fn load_plugins(&mut self, plugin_names: &HashSet<String>) -> bool {
        if !self.iniparser.load(self.inipathname) {
            println!("❌ Failed loading inifile from {:?}", self.inipathname);
            return false;
        }
        for name in plugin_names {
            let lib_name = format!("lib{}_plugin.{}", name.to_lowercase(), LIB_EXT);
            let path = Path::new(self.pluginsdirpath).join(&lib_name);
            println!("🔍 Loading plugin: {:?}", path);

            let library = match unsafe { Library::new(&path) } {
                Ok(lib) => lib,
                Err(err) => {
                    eprintln!("❌ Failed to load {}: {}", lib_name, err);
                    return false;
                }
            };

            // load entry function
            let entry: Symbol<PluginCreateFn> = match unsafe { library.get(b"pluginEntry") } {
                Ok(sym) => sym,
                Err(err) => {
                    eprintln!("❌ {}: missing pluginEntry: {}", lib_name, err);
                    return false;
                }
            };

            let handle = unsafe { entry() };
            if handle.ptr.is_null() {
                eprintln!("❌ {}: pluginEntry returned null handle", lib_name);
                return false;
            }

            // optional pluginExit
            let exit_fn: Option<unsafe extern "C" fn(*mut PluginHandle)> =
                unsafe { library.get(b"pluginExit").ok().map(|s| *s) };

            // retrieve data from inifile and send to it to plugin
            if let Some(section) = self.iniparser.get_resolved_section(name, INI_SEARCH_DEPTH) {
                if unsafe { !(handle.set_params)(handle.ptr, &section) } {
                    return false;
                }
            }

            // Box it and store as raw pointer
            let boxed_handle = Box::new(handle);
            let handle_ptr = Box::into_raw(boxed_handle);

            self.plugins.insert(
                name.clone(),
                PluginDescriptor {
                    handle: handle_ptr,
                    _lib: library,
                    exit_fn,
                },
            );

            println!("✅ Loaded plugin: {}", name);
        }
        true
    }

    pub fn enable_plugins(&mut self) -> bool {
        println!("⚡ Enabling plugins...");
        for (name, descriptor) in &self.plugins {
            unsafe {
                let handle: &mut PluginHandle = &mut *descriptor.handle;
                if !plugin_do_enable(handle) {
                    return false;
                }
                println!("✅ Enabled plugin: {}", name);
            }
        }
        true
    }

    pub fn unload_plugins(&mut self) {
        for (name, descriptor) in self.plugins.drain() {
            unsafe {
                let handle = &mut *descriptor.handle;

                // call pluginExit if available
                if let Some(exit) = descriptor.exit_fn {
                    exit(handle);
                }

                // call destroy
                (handle.destroy)(handle.ptr);

                // drop boxed handle
                let _ = Box::from_raw(descriptor.handle);
            }
            println!("🗑️ Unloaded plugin: {}", name);
        }
    }
}

impl Drop for PluginManager {
    fn drop(&mut self) {
        self.unload_plugins();
    }
}
