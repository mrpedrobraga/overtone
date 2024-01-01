use super::errors::OvertoneApiError;
use super::project::Project;
use crate::serialization::project::PluginDependencyEntry;
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
    fn on_plugin_load(&mut self, _project: &Project) {}
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
    lib: Library,
}

pub const PLUGIN_GETTER_SYMBOL: &'static [u8; 10] = b"get_plugin";

impl<'a> LoadedPlugin<'a> {
    pub fn get_uid(&self) -> &PluginId {
        return &self.uid;
    }

    pub fn get_plugin(&self) -> &Box<dyn Plugin> {
        return &self.plugin;
    }

    pub fn get_lib(&'a self) -> &'a Library {
        return &self.lib;
    }

    pub fn from_dependency_decl(
        base_path: &Option<PathBuf>,
        source: &'a PluginDependencyEntry,
    ) -> Result<LoadedPlugin<'a>, OvertoneApiError> {
        let path = base_path.as_ref().map_or_else(
            || PathBuf::from(source.path.clone()),
            |b_p| b_p.join(source.path.clone()),
        );

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

        Ok(LoadedPlugin {
            uid: PluginId::new(),
            lib,
            source,
            plugin,
        })
    }
}

impl<'a> Debug for LoadedPlugin<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("[Plugin '{}']", self.plugin.get_name()).as_str())
    }
}
