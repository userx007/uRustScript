use libloading::{Library, Symbol};
use plugin_api::PluginInterface;
use std::collections::HashSet;
use std::path::Path;

pub type PluginCreateFn = unsafe fn() -> *mut dyn PluginInterface;
pub type PluginDestroyFn = unsafe fn(*mut dyn PluginInterface);

pub struct LoadedPlugin {
    pub library: Library,
    pub plugin: *mut dyn PluginInterface,
}

pub fn load_plugins(plugin_names: &HashSet<String>, plugin_dir: &str) -> Vec<LoadedPlugin> {
    let mut plugins = Vec::new();

    for name in plugin_names {
        let path = Path::new(plugin_dir).join(name);
        unsafe {
            let lib = Library::new(&path).unwrap();
            let create: Symbol<PluginCreateFn> = lib.get(b"plugin_create").unwrap();
            let plugin = create();

            plugins.push(LoadedPlugin {
                library: lib,
                plugin,
            });
        }
    }

    plugins
}

pub fn unload_plugins(plugins: Vec<LoadedPlugin>) {
    for loaded in plugins {
        unsafe {
            let destroy: Symbol<PluginDestroyFn> = loaded.library.get(b"plugin_destroy").unwrap();
            destroy(loaded.plugin);
        }
    }
}
