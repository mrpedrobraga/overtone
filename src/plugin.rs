use crate::serialization::project::PluginDependencyEntry;

use super::errors::OvertoneApiError;
use super::project::Project;
use libloading::Library;
use std::fmt::Debug;
use std::path::PathBuf;

#[allow(dead_code)]

/// An Overtone plugin, which will be loaded, registered,
/// and can contribute with Renderers, Track Fragments and more.
pub trait Plugin {
    /// Returns the 'id' of the plugin, which will identify it from other plugins
    /// To avoid 'id' collision, try to be unique.
    /// Note that as the plugin is loaded, it'll be identified by it's uid instead.
    fn get_id(&self) -> &'static str;
    /// Returns the name of the plugin, which will be displayed when errors occur.
    fn get_name(&self) -> &'static str {
        return "a";
    }
    /// Signal executed when the plugin loads.
    fn on_plugin_load(&self, _project: Project) {}
}

/// Internal type of a plugin identifier.
pub struct PluginIdType(());
/// Id of a loaded plugin.
pub type PluginId = uid::Id<PluginIdType>;
// Type of a function that retrieves a plugin from a library.
pub type PluginGetterFn = unsafe fn() -> Box<dyn Plugin>;

pub struct LoadedPlugin<'a> {
    pub uid: PluginId,
    pub plugin: Box<dyn Plugin>,
    pub source: &'a PluginDependencyEntry,

    // This must be declared last
    // as it needs to be dropped after 'plugin' drops.
    pub lib: Library,
}

pub const PLUGIN_GETTER_SYMBOL: &'static [u8; 10] = b"get_plugin";

impl<'a> LoadedPlugin<'a> {
    pub fn from_external_reference(
        base_path: &Option<PathBuf>,
        plugin_ref: &'a PluginDependencyEntry,
    ) -> Result<LoadedPlugin<'a>, OvertoneApiError> {
        let path = match base_path {
            None => PathBuf::from(plugin_ref.path.clone()),
            Some(v) => v.join(plugin_ref.path.clone()),
        };

        let lib: libloading::Library;
        let plugin: Box<dyn Plugin>;
        unsafe {
            let l = libloading::Library::new(path);
            lib = match l {
                Ok(l) => l,
                Err(e) => return Err(OvertoneApiError::LibraryNotFound(e)),
            };
            plugin = match lib.get::<PluginGetterFn>(PLUGIN_GETTER_SYMBOL) {
                Ok(v) => v(),
                Err(_) => return Err(OvertoneApiError::LibraryIsNotOvertonePlugin()),
            };
        }
        let loaded = LoadedPlugin {
            uid: PluginId::new(),
            lib,
            source: plugin_ref,
            plugin,
        };
        Ok(loaded)
    }
}

impl<'a> Debug for LoadedPlugin<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("[Plugin '{}']", self.plugin.get_name()).as_str())
    }
}
