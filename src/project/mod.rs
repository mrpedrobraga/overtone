use crate::{
    arrangement::serialization::ArrangementHeader,
    plugin::{dependency::PluginDependencyEntry, errors::PluginError},
    utils::containers::PushReturn,
};
pub mod errors;
pub mod serialization;
use self::serialization::{
    load_project_deps_from_directory, load_project_from_directory, ProjectFile,
};
use super::{api::errors::OvertoneApiError, plugin::PluginBox, utils::info::Info};
use std::path::PathBuf;

/// Overtone Project, holds references to in-disk dependencies and manages
/// changes upon them (refactoring). It also contains loaded references to
/// the dependencies, and manages loading/unloading them.
#[derive(Debug)]
pub struct Project<'a> {
    pub file: ProjectFile,
    pub base_path: Option<PathBuf>,

    pub loaded_plugins: Vec<PluginBox<'a>>,
    pub dependencies: ProjectDependencies,
}

#[derive(Debug)]
pub struct ProjectDependencies {
    pub arrangements: Vec<ArrangementHeader>,
}

impl<'a> Info for Project<'a> {
    fn get_name(&self) -> &str {
        self.file.info.name.as_str()
    }
}

impl<'a> Project<'a> {
    pub fn new(file: ProjectFile) -> Self {
        Self {
            file,
            base_path: None,
            loaded_plugins: Vec::new(),
            dependencies: ProjectDependencies {
                arrangements: vec![],
            },
        }
    }

    // Loads an overtone project from a directory, looking for an `Overtone.toml` file.
    pub fn load_from_directory<S: Into<String>>(path: S) -> Result<Self, OvertoneApiError> {
        let path_str: String = path.into();
        let file = load_project_from_directory(&path_str)?;

        let dependencies = load_project_deps_from_directory(&path_str, &file.path_overrides)?;

        Ok(Project {
            file,
            base_path: Some(PathBuf::from(path_str)),
            loaded_plugins: vec![],
            dependencies,
        })
    }

    pub fn get_plugins(&self) -> &Option<Vec<PluginDependencyEntry>> {
        &self.file.plugins
    }

    pub fn iter_loaded_plugins(&'a self) -> std::slice::Iter<'a, PluginBox<'a>> {
        self.loaded_plugins.iter()
    }

    // Loads a plugin from a shared library located at the designated relative path.
    pub fn load_plugin(&'a mut self, id: String) -> Result<&'a PluginBox<'a>, PluginError> {
        if let Some(_v) = self.loaded_plugins.iter().find(|p| p.source.id == id) {
            return Err(PluginError::PluginAlreadyLoaded());
        }

        let plugins = match &self.file.plugins {
            None => return Err(PluginError::MissingPlugin(id)),
            Some(v) => v,
        };
        let plugin_ref = plugins.iter().find(|plug_ref| plug_ref.id == id);
        let plugin_ref = match plugin_ref {
            None => return Err(PluginError::MissingPlugin(id)),
            Some(p) => p,
        };

        let loaded: PluginBox = PluginBox::from_dependency_decl(&self.base_path, plugin_ref)?;

        // TODO: Call the `on_plugin_load` callback passing a view to the project.
        //loaded.plugin.on_plugin_load(self);

        let loaded: &PluginBox = self.loaded_plugins.push_and_get(loaded);

        Ok(loaded)
    }
}
