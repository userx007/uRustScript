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
            let path = Path::new(self.pluginsdirpath).join(lib_name);
            println!("Loading plugin: {:?}", path);

            unsafe {
                let library = Library::new(&path).unwrap();
                let create: Symbol<PluginCreateFn> = library.get(b"plugin_create").unwrap();
                let handle = create(); // type PluginHandle

                // retrieve data from inifile and send to it to plugin
                if let Some(section) = self.iniparser.get_resolved_section(name, INI_SEARCH_DEPTH) {
                    if !(handle.set_params)(handle.ptr, &section) {
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
                    },
                );
            }
        }
        true
    }

    pub fn enable_plugins(&mut self) -> bool {
        println!("Enabling plugins");
        for descriptor in self.plugins.values() {
            unsafe {
                let handle: &mut PluginHandle = &mut *descriptor.handle;
                if !plugin_do_enable(handle) {
                    return false;
                }
            }
        }
        true
    }

    fn unload_plugin(&mut self, name: &str) {
        if let Some(descriptor) = self.plugins.remove(name) {
            unsafe {
                if !descriptor.handle.is_null() {
                    // Call destroy
                    ((*descriptor.handle).destroy)((*descriptor.handle).ptr);
                    // Drop boxed handle
                    let _ = Box::from_raw(descriptor.handle);
                }
            }
            println!("🗑️ Unloaded plugin {}", name);
        }
    }

    fn unload_plugins(&mut self) {
        let plugin_names: Vec<String> = self.plugins.keys().cloned().collect();
        for name in plugin_names {
            self.unload_plugin(&name);
        }
    }
}

impl Drop for PluginManager {
    fn drop(&mut self) {
        self.unload_plugins();
    }
}
