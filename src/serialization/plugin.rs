use crate::{errors::OvertoneApiError, plugin::Plugin};
use libloading::Library;
use std::path::PathBuf;

// Type of a function that retrieves a plugin from a library.
pub type PluginGetterFn = unsafe fn() -> Box<dyn Plugin>;

pub const PLUGIN_GETTER_SYMBOL: &'static [u8; 10] = b"get_plugin";

pub fn load_plugin_lib(path: PathBuf) -> Result<(Library, Box<dyn Plugin>), OvertoneApiError> {
    let lib: libloading::Library;
    let plugin: Box<dyn Plugin>;
    unsafe {
        let l = libloading::Library::new(path);
        lib = l.map_err(|e| OvertoneApiError::LibraryNotFound(e))?;
        let plugin_getter = lib
            .get::<PluginGetterFn>(PLUGIN_GETTER_SYMBOL)
            .map_err(|_| OvertoneApiError::LibraryIsNotOvertonePlugin())?;
        plugin = plugin_getter();
    }
    Ok((lib, plugin))
}
