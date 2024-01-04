use self::dependency::PluginDependencyEntry;
use self::errors::PluginError;
use self::serialization::load_plugin_lib;

use super::project::Project;
use libloading::Library;
use std::fmt::Debug;
use std::path::PathBuf;

pub mod dependency;
pub mod errors;
pub mod serialization;

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
pub type PluginId = i32;

pub struct LoadedPlugin<'a> {
    pub uid: PluginId,
    pub plugin: Box<dyn Plugin>,
    pub source: &'a PluginDependencyEntry,

    // This must be declared last
    // as it needs to be dropped after 'plugin' drops.
    lib: Library,
}

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
    ) -> Result<LoadedPlugin<'a>, PluginError> {
        let path = base_path.as_ref().map_or_else(
            || PathBuf::from(source.path.clone()),
            |b_p| b_p.join(source.path.clone()),
        );

        let (lib, plugin) = load_plugin_lib(path)?;

        Ok(LoadedPlugin {
            uid: 0,
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
