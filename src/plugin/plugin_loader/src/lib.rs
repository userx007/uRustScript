use plugin_interface::*;
use libloading::{Library, Symbol};
use std::collections::HashSet;
use std::path::PathBuf;

pub struct LoadedPlugin {
    pub name: String,
    pub lib: Library,      // Keeps the .so / .dll alive
    pub plugin: Plugin,    // The plugin instance (with vtable)
}

pub fn load_plugins(plugin_names: &HashSet<String>, plugin_dir: &str) -> Vec<LoadedPlugin> {
    let mut loaded = Vec::new();

    for name in plugin_names {
        // Compose full path, e.g. "plugins/libutils_plugin.so"
        let mut path = PathBuf::from(plugin_dir);
        path.push(name);

        if !path.exists() {
            eprintln!("⚠️  Plugin not found: {}", path.display());
            continue;
        }

        unsafe {
            match Library::new(&path) {
                Ok(lib) => {
                    // Load the required symbols
                    let create: Symbol<PluginCreateFn> =
                        match lib.get(b"create_plugin") {
                            Ok(s) => s,
                            Err(e) => {
                                eprintln!("⚠️  {}: missing create_plugin: {e}", path.display());
                                continue;
                            }
                        };

                    let destroy: Symbol<PluginDestroyFn> =
                        match lib.get(b"destroy_plugin") {
                            Ok(s) => s,
                            Err(e) => {
                                eprintln!("⚠️  {}: missing destroy_plugin: {e}", path.display());
                                continue;
                            }
                        };

                    // Create the plugin instance
                    let plugin = create();

                    println!("✅ Loaded plugin: {}", name);
                    loaded.push(LoadedPlugin {
                        name: name.clone(),
                        lib, // keep library alive
                        plugin,
                    });
                }
                Err(e) => {
                    eprintln!("❌ Failed to load {}: {e}", path.display());
                }
            }
        }
    }

    loaded
}


pub fn unload_plugins(plugins: Vec<LoadedPlugin>) {
    for mut loaded in plugins {
        unsafe {
            let destroy_symbol: Result<Symbol<PluginDestroyFn>, _> =
                loaded.lib.get(b"destroy_plugin");

            if let Ok(destroy) = destroy_symbol {
                (*destroy)(&mut loaded.plugin as *mut _);
            }
        }
        println!("🧹 Unloaded plugin: {}", loaded.name);
        // `Library` automatically unloads when dropped
    }
}

/*
use std::collections::HashSet;

fn main() {
    let mut plugin_names = HashSet::new();
    plugin_names.insert("libutils_plugin.so".to_string());
    plugin_names.insert("libmath_plugin.so".to_string());

    let plugins = load_plugins(&plugin_names, "./plugins");

    // Call plugin methods
    unsafe {
        for p in &plugins {
            ((*p.plugin.vtable).do_init)(p.plugin.instance, std::ptr::null_mut());
            ((*p.plugin.vtable).do_dispatch)(
                p.plugin.instance,
                b"RUN\0".as_ptr() as _,
                b"test\0".as_ptr() as _,
            );
            let data_ptr = ((*p.plugin.vtable).get_data)(p.plugin.instance);
            let data = std::ffi::CStr::from_ptr(data_ptr).to_string_lossy();
            println!("Plugin {} returned: {}", p.name, data);
        }
    }

    unload_plugins(plugins);
}
*/