use crate::plugin::Plugin;
use libloading::Library;
use std::path::PathBuf;

use super::errors::PluginError;

// Type of a function that retrieves a plugin from a library.
pub type PluginGetterFn = unsafe fn() -> Box<dyn Plugin>;

pub const PLUGIN_GETTER_SYMBOL: &[u8; 19] = b"get_overtone_plugin";

pub fn load_plugin_lib(path: PathBuf) -> Result<(Library, Box<dyn Plugin>), PluginError> {
    let lib: libloading::Library;
    let plugin: Box<dyn Plugin>;
    unsafe {
        let l = libloading::Library::new(path);
        lib = l.map_err(PluginError::LibraryNotFound)?;
        let plugin_getter = lib
            .get::<PluginGetterFn>(PLUGIN_GETTER_SYMBOL)
            .map_err(|_| PluginError::LibraryIsNotOvertonePlugin())?;
        plugin = plugin_getter();
    }
    Ok((lib, plugin))
}
