//! # Plugins
//!
//! Overtone is, more than anything, a host for orchestrating concepts
//! defined in third-party plugins. Even the "builtin" functionality is
//! implemented through a plugin (`music-core`). This is similar to how
//! programming languages can have multiple libraries.
//!
//! A [`Plugin`] comes with metadata and offers [`PluginContributions`]
//! that can offer specific kinds of functionalities.

use super::project::Project;
use libloading::Library;
use std::fmt::Debug;
use std::path::PathBuf;
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::{DependencyId, OvertoneError};
use crate::project::DependencyEntry;
use crate::renderer::RenderExporter;
use crate::renderer::Renderer;

/// Unique identifier of a loaded plugin.
pub type PluginId = i32;

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
        "Unnamed Plugin"
    }

    /// Signal executed when the plugin loads.
    fn on_plugin_load(&mut self, _project: &Project) {}

    /// Get all the plugin contributions
    fn get_contributions(&self) -> PluginContributions;
}

#[derive(Serialize, Deserialize, Debug)]
/// Trait that describes a plugin from the perspective of a project.
pub struct PluginDependencyEntry {
    pub id: DependencyId,
    pub path: PathBuf,
}

impl DependencyEntry for PluginDependencyEntry {
    fn get_id(&self) -> DependencyId {
        self.id.clone()
    }

    fn get_path(&self) -> PathBuf {
        self.path.clone()
    }
}

/// Struct that holds all the contributions from a plugin.
pub struct PluginContributions {
    pub renderers: Option<HashMap<String, Box<dyn Renderer>>>,
    pub exporters: Option<HashMap<String, Box<dyn RenderExporter>>>,
}

/// Type that holds a plugin loaded from a foreign library, metadata,
/// and the loaded library itself.
pub struct LoadedPlugin<'a> {
    pub uid: PluginId,
    pub plugin: Box<dyn Plugin>,
    pub source: &'a PluginDependencyEntry,

    // This must be declared last
    // as it needs to be dropped after 'plugin' drops.
    lib: Library,
}

impl<'a> LoadedPlugin<'a> {
    /// Returns the plugin's unique identifier.
    pub fn get_uid(&self) -> &PluginId {
        &self.uid
    }

    /// Returns a reference to the [`Plugin`] itself.
    pub fn get_plugin(&self) -> &dyn Plugin {
        self.plugin.as_ref()
    }

    /// Returns a reference to the [`Library`] the plugin was loaded from.
    pub fn get_lib(&'a self) -> &'a Library {
        &self.lib
    }

    /// Constructor. This loads a plugin from a dependency entry (which
    /// might contain an absolute or relative path.
    pub fn load_from_dependency_entry(
        base_path: &Option<PathBuf>,
        source: &'a PluginDependencyEntry,
    ) -> Result<LoadedPlugin<'a>, PluginError> {
        pub type PluginProvider = unsafe fn() -> Box<dyn Plugin>;
        pub const PLUGIN_PROVIDER_NAME: &[u8; 19] = b"get_overtone_plugin";

        let path = base_path
            .as_ref()
            .map_or_else(|| source.path.clone(), |b_p| b_p.join(source.path.clone()));

        let lib: libloading::Library;
        let plugin: Box<dyn Plugin>;
        unsafe {
            let l = libloading::Library::new(path);
            lib = l.map_err(PluginError::LibraryNotFound)?;
            let plugin_getter = lib
                .get::<PluginProvider>(PLUGIN_PROVIDER_NAME)
                .map_err(|_| PluginError::LibraryIsNotOvertonePlugin())?;
            plugin = plugin_getter();
        }

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

/// This macro is useful for third-parties creating new plugins in new rust projects.
///
/// A plugin library uses a naming scheme to make the "give me the plugin" function
/// visible. Since there is no stable ABI or official way to make a library plugin,
/// this macro helps plugins be "in sync" with the version of overtone they were
/// compiled for.
#[macro_export]
macro_rules! overtone_plugin {
    ( $e: expr ) => {
        #[no_mangle]
        pub fn get_overtone_plugin() -> Box<dyn $crate::plugin::Plugin> { $e }
    }
}

//MARK: Errors

#[derive(Debug)]
/// An error originated from some operation regarding plugins.
pub enum PluginError {
    /// Plugin was already loaded and shouldn't be loaded again.
    PluginAlreadyLoaded(),
    /// A plugin, which was referred by this id, does not exist.
    MissingPlugin(String),
    /// Library couldn't be loaded for some reason.
    LibraryNotFound(libloading::Error),
    /// Library was loaded but is not recognised as an Overtone plugin.
    LibraryIsNotOvertonePlugin(),
}

impl From<PluginError> for OvertoneError {
    fn from(value: PluginError) -> Self {
        OvertoneError::PluginError(value)
    }
}