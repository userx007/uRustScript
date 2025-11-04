use libloading::{Library, Symbol};
use plugin_api::{PluginCreateFn, PluginHandle};
use std::collections::HashSet;
use std::path::Path;

#[cfg(target_os = "windows")]
const LIB_EXT: &str = "dll";

#[cfg(target_os = "linux")]
const LIB_EXT: &str = "so";

#[cfg(target_os = "macos")]
const LIB_EXT: &str = "dylib";


pub struct LoadedPlugin {
    pub library: Library,
    pub handle: PluginHandle,
}

pub fn build_library_name(name: &str) -> String {
    let name = name.to_lowercase();
    format!("lib{0}_plugin.{1}", name, LIB_EXT)
}


pub fn load_plugins(plugin_names: &HashSet<String>, plugin_dir: &str) -> Vec<LoadedPlugin> {
    let mut plugins = Vec::new();

    for name in plugin_names {
        let lib_name = build_library_name(name);
        let path = Path::new(plugin_dir).join(lib_name);
        println!("Loading plugin: {:?}", path);
        unsafe {
            let library = Library::new(&path).unwrap();
            let create: Symbol<PluginCreateFn> = library.get(b"plugin_create").unwrap();
            let handle = create(); // <-- returns PluginHandle now
            plugins.push(LoadedPlugin { library, handle });
        }
    }
    plugins
}


pub fn unload_plugins(plugins: Vec<LoadedPlugin>) {
    for loaded in plugins {
        (loaded.handle.destroy)(loaded.handle.ptr);
    }
}
