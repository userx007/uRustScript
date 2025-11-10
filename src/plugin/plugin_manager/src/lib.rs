use libloading::{Library, Symbol};
use plugin_api::{PluginCreateFn, PluginHandle};
use std::collections::{HashMap, HashSet};
use std::path::Path;

#[cfg(target_os = "windows")]
const LIB_EXT: &str = "dll";

#[cfg(target_os = "linux")]
const LIB_EXT: &str = "so";

#[cfg(target_os = "macos")]
const LIB_EXT: &str = "dylib";

pub struct PluginDescriptor {
    pub handle: *mut PluginHandle,
    pub _lib: Library, // underscore means “used to hold lifetime”
}

pub struct PluginManager {
    pluginsdirpath: &'static str,
    pub plugins: HashMap<String, PluginDescriptor>,
}

impl PluginManager {
    pub fn new(pluginsdirpath: &'static str) -> Self {
        Self {
            pluginsdirpath,
            plugins: HashMap::new(),
        }
    }

    pub fn load_plugins(&mut self, plugin_names: &HashSet<String>) {
        for name in plugin_names {
            let lib_name = format!("lib{}_plugin.{}", name.to_lowercase(), LIB_EXT);
            let path = Path::new(self.pluginsdirpath).join(lib_name);
            println!("Loading plugin: {:?}", path);

            unsafe {
                let library = Library::new(&path).unwrap();
                let create: Symbol<PluginCreateFn> = library.get(b"plugin_create").unwrap();
                let handle = create(); // type PluginHandle

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
    }

    pub fn unload_plugin(&mut self, name: &str) {
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

    pub fn unload_plugins(&mut self) {
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