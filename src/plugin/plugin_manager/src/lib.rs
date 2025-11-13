use libloading::{Library, Symbol};
use std::collections::{HashMap, HashSet};
use std::path::Path;

use utils::ini_parser::IniParserEx;
use plugin_api::PluginHandle;


#[cfg(target_os = "windows")]
const LIB_EXT: &str = "dll";

#[cfg(target_os = "linux")]
const LIB_EXT: &str = "so";

#[cfg(target_os = "macos")]
const LIB_EXT: &str = "dylib";

const INI_SEARCH_DEPTH: usize = 5;

/// Information about a dynamically loaded plugin
pub struct PluginDescriptor {
    pub handle: *mut PluginHandle,
    pub _lib: Library, // underscore means “used to hold lifetime”
    pub exit_fn: Option<unsafe extern "C" fn(*mut PluginHandle)>,
}

/// Manages the discovery, loading, and lifetime of plugins.
pub struct PluginManager {
    pluginsdirpath: &'static str,
    inipathname: &'static str,
    iniparser: IniParserEx,
    pub plugins: HashMap<String, PluginDescriptor>,
}

impl PluginManager {
    /// Create a new plugin manager with a directory path
    pub fn new(pluginsdirpath: &'static str, inipathname: &'static str) -> Self {
        Self {
            pluginsdirpath,
            inipathname,
            iniparser: IniParserEx::default(),
            plugins: HashMap::new(),
        }
    }

    /// Load all requested plugins, initialize and configure them
    pub fn load_plugins(&mut self, plugins: &HashSet<String>) -> bool {
        println!("🔍 Loading plugins from {:?}", self.pluginsdirpath);

        if !self.iniparser.load(self.inipathname) {
            eprintln!("❌ Failed loading inifile from {:?}", self.inipathname);
            return false;
        }

        for name in plugins {
            let lib_name = format!("lib{}_plugin.{}", name.to_lowercase(), LIB_EXT);
            let lib_path = Path::new(self.pluginsdirpath).join(lib_name);
            if !Path::new(&lib_path).exists() {
                eprintln!("❌ Missing plugin library: {:?}", lib_path);
                return false;
            }

            // Attempt to open the shared library
            let library = match unsafe { Library::new(&lib_path) } {
                Ok(lib) => lib,
                Err(err) => {
                    eprintln!("❌ Failed to load {:?}: {}", lib_path, err);
                    return false;
                }
            };
println!("1");
            // Load plugin entry symbol
            let entry: Symbol<unsafe extern "C" fn() -> *mut PluginHandle> =
                match unsafe { library.get(b"pluginEntry") } {
                    Ok(symbol) => symbol,
                    Err(err) => {
                        eprintln!("❌ {:?}: missing pluginEntry: {}", lib_path, err);
                        return false;
                    }
                };
println!("2");
            // Call entry to create plugin handle
            let handle_ptr = unsafe { entry() };
            if handle_ptr.is_null() {
                eprintln!("❌ {:?}: pluginEntry returned null handle", lib_path);
                return false;
            }

            // Load optional exit function
            let exit_fn: Option<unsafe extern "C" fn(*mut PluginHandle)> =
                unsafe { library.get(b"pluginExit").ok().map(|s| *s) };
println!("3");
            // Set parameters from INI
            if let Some(section) = self.iniparser.get_resolved_section(name, INI_SEARCH_DEPTH) {
                println!("⚙️  Applying INI section for '{}': {:?}", name, section);
                unsafe {
                    if !((*handle_ptr).set_params)((*handle_ptr).ptr, &section) {
                        eprintln!("❌ {}: set_params() failed", name);
                        // Call pluginExit to cleanup
                        if let Some(exit_fn) = exit_fn {
                            exit_fn(handle_ptr);
                        }
                        return false;
                    }
                }
            }
println!("4");
            unsafe {
                if !((*handle_ptr).do_init)((*handle_ptr).ptr, std::ptr::null_mut()) {
                    eprintln!("Plugin initialization failed");
                    return false;
                }
            }
println!("5");
            println!("✅ Loaded plugin: {}", name);

            // Register in plugin table
            self.plugins.insert(
                name.clone(),
                PluginDescriptor {
                    handle: handle_ptr,
                    _lib: library,
                    exit_fn,
                },
            );
        }

        true
    }

    pub fn enable_plugins(&mut self) -> bool {
        println!("⚡ Enabling plugins...");

        for (name, descriptor) in &self.plugins {
            unsafe {
                // Always check for null pointer before dereferencing
                if descriptor.handle.is_null() {
                    eprintln!("❌ Plugin '{}' has null handle!", name);
                    return false;
                }

                let handle = &mut *descriptor.handle;

                // Call the plugin’s enable function directly via FFI
                (handle.do_enable)(handle.ptr);

                // Verify it actually marked itself enabled
                if !(handle.is_enabled)(handle.ptr) {
                    eprintln!("❌ Plugin '{}' failed to enable!", name);
                    return false;
                }

                println!("✅ Enabled plugin: {}", name);
            }
        }

        true
    }


    /// Cleanup and unload all loaded plugins
    pub fn unload_all(&mut self) {
        for (name, descriptor) in self.plugins.drain() {
            println!("🧹 Unloading plugin '{}'", name);
            unsafe {
                if let Some(exit_fn) = descriptor.exit_fn {
                    exit_fn(descriptor.handle);
                } else if !descriptor.handle.is_null() {
                    // fallback if no exit_fn defined
                    ((*descriptor.handle).destroy)((*descriptor.handle).ptr);
                }
            }
        }
    }
}

impl Drop for PluginManager {
    fn drop(&mut self) {
        self.unload_all();
    }
}
