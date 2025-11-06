use libloading::{Library, Symbol};
use plugin_api::{PluginCreateFn, PluginIdentifier};
use std::collections::{HashMap,HashSet};
use std::path::Path;

#[cfg(target_os = "windows")]
const LIB_EXT: &str = "dll";

#[cfg(target_os = "linux")]
const LIB_EXT: &str = "so";

#[cfg(target_os = "macos")]
const LIB_EXT: &str = "dylib";


pub fn build_library_name(name: &str) -> String {
    let name = name.to_lowercase();
    format!("lib{0}_plugin.{1}", name, LIB_EXT)
}

pub fn load_plugins(plugin_names: &HashSet<String>, plugin_dir: &str) -> PluginIdentifier {
    let mut plugins: PluginIdentifier = HashMap::new();

    for name in plugin_names {
        let lib_name = build_library_name(name);
        let path = Path::new(plugin_dir).join(lib_name);
        println!("Loading plugin: {:?}", path);
        unsafe {
            let library = Library::new(&path).unwrap();
            let create: Symbol<PluginCreateFn> = library.get(b"plugin_create").unwrap();
            let handle = create(); // type PluginHandle
            plugins.insert(name.to_string(), (handle,library));
        }
    }
    plugins
}

pub fn unload_plugins(plugins: PluginIdentifier) {
    for (_, (handle, _)) in plugins {
        (handle.destroy)(handle.ptr);
    }
}
