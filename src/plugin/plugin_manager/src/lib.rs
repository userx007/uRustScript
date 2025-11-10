use libloading::{Library, Symbol};
use plugin_api::{PluginCreateFn, PluginHandle};
use std::collections::{HashMap, HashSet};
use std::path::Path;
use std::ffi::c_void;

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

pub type LoadedPlugins = HashMap<String, PluginDescriptor>;

pub struct PluginManager {
    pub plugins: LoadedPlugins,
}

impl PluginManager {
    pub fn new() -> Self {
        Self { plugins: HashMap::new() }
    }

    pub fn load_plugins(&mut self, plugin_names: &HashSet<String>, plugin_dir: &str) {

        for name in plugin_names {
            let lib_name = self.build_library_name(name);
            let path = Path::new(plugin_dir).join(lib_name);
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
                    PluginDescriptor { handle: handle_ptr, _lib: library },
                );
            }
        }
    }

    pub fn unload_plugin(&mut self, name: &str) {
        if let Some(loaded) = self.plugins.remove(name) {
            println!("🧹 Unloading plugin: {}", name);
            unsafe {
                // Reclaim Box to drop it safely
                let boxed_handle = Box::from_raw(loaded.handle);

                // Call the plugin’s destroy function
                if !boxed_handle.ptr.is_null() {
                    (boxed_handle.destroy)(boxed_handle.ptr);
                }

                // Library drops here -> automatically unloaded
            }
        } else {
            println!("⚠️ Tried to unload non-existent plugin: {}", name);
        }
    }

    pub fn unload_plugins(&mut self) {
        let plugin_names: Vec<String> = self.plugins.keys().cloned().collect();
        for name in plugin_names {
            self.unload_plugin(&name);
        }
    }

    pub fn get_ptr(&self, name: &str) -> Option<*mut c_void> {
        self.plugins
            .get(name)
            .map(|p| unsafe { (*p.handle).ptr })
    }

    pub fn build_library_name(&self, name: &str) -> String {
        let name = name.to_lowercase();
        format!("lib{0}_plugin.{1}", name, LIB_EXT)
    }

}

