use contributions::PluginContributions;

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
pub mod contributions;

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
        "unidentified plugin"
    }

    /// Signal executed when the plugin loads.
    fn on_plugin_load(&mut self, _project: &Project) {}

    /// Get all the plugin contributions
    fn get_contributions(&self) -> PluginContributions;
}

/// Internal type of a plugin identifier.
pub struct PluginIdType(());

/// Id of a loaded plugin.
pub type PluginId = i32;

pub struct PluginBox<'a> {
    pub uid: PluginId,
    pub plugin: Box<dyn Plugin>,
    pub source: &'a PluginDependencyEntry,

    // This must be declared last
    // as it needs to be dropped after 'plugin' drops.
    lib: Library,
}

impl<'a> PluginBox<'a> {
    pub fn get_uid(&self) -> &PluginId {
        &self.uid
    }

    pub fn get_plugin(&self) -> &dyn Plugin {
        self.plugin.as_ref()
    }

    pub fn get_lib(&'a self) -> &'a Library {
        &self.lib
    }

    pub fn from_dependency_decl(
        base_path: &Option<PathBuf>,
        source: &'a PluginDependencyEntry,
    ) -> Result<PluginBox<'a>, PluginError> {
        let path = base_path
            .as_ref()
            .map_or_else(|| source.path.clone(), |b_p| b_p.join(source.path.clone()));

        let (lib, plugin) = load_plugin_lib(path)?;

        Ok(PluginBox {
            uid: 0,
            lib,
            source,
            plugin,
        })
    }
}

impl<'a> Debug for PluginBox<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("[Plugin '{}']", self.plugin.get_name()).as_str())
    }
}

/// Declares a new Overtone [`Plugin`] instance, with everything it might do.
#[macro_export]
macro_rules! overtone_plugin {
    ( $e: expr ) => {
        #[no_mangle]
        pub fn get_overtone_plugin() -> Box<dyn $crate::plugin::Plugin> { $e }
    }
}