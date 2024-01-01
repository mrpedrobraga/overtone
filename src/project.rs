use crate::serialization::project::{load_project_file, PluginDependencyEntry, ProjectFile};

use super::{errors::OvertoneApiError, info::Info, plugin::LoadedPlugin, utils::PushReturn};
use std::{fs, path::PathBuf};

#[derive(Debug)]
pub struct Project<'a> {
    pub file: ProjectFile,
    pub base_path: Option<PathBuf>,
    pub loaded_plugins: Vec<LoadedPlugin<'a>>,
}

impl<'a> Info for Project<'a> {
    fn get_name(&self) -> &str {
        return self.file.info.name.as_str();
    }
}

const OVERTONE_PROJECT_FILE_NAME: &'static str = "Overtone.toml";

impl<'a> Project<'a> {
    pub fn new(file: ProjectFile) -> Self {
        Self {
            file,
            base_path: None,
            loaded_plugins: Vec::new(),
        }
    }

    // Loads an overtone project from a directory, looking for an `Overtone.toml` file.
    pub fn load_from_directory<S: Into<String>>(path: S) -> Result<Self, OvertoneApiError> {
        let path_str: String = path.into();
        let dir = match fs::read_dir(path_str.clone()) {
            Ok(v) => v,
            Err(e) => return Err(OvertoneApiError::DirectoryNotFound(e)),
        };

        let dir_entry = dir.into_iter().find(|e| match e {
            Err(_) => false,
            Ok(v) => v.file_name() == OVERTONE_PROJECT_FILE_NAME,
        });
        let dir_entry = match dir_entry {
            None => return Err(OvertoneApiError::DirectoryIsNotOvertoneProject(None)),
            Some(v) => match v {
                Err(e) => return Err(OvertoneApiError::DirectoryIsNotOvertoneProject(Some(e))),
                Ok(v) => v,
            },
        };
        let file = load_project_file(dir_entry.path())?;

        Ok(Project {
            file,
            base_path: Some(PathBuf::from(path_str)),
            loaded_plugins: vec![],
        })
    }

    pub fn get_plugins(&self) -> &Option<Vec<PluginDependencyEntry>> {
        &self.file.plugins
    }

    pub fn iter_loaded_plugins(&'a self) -> std::slice::Iter<'a, LoadedPlugin<'a>> {
        self.loaded_plugins.iter()
    }

    // Loads a plugin from a library located at its path.
    pub fn load_plugin(&'a mut self, id: String) -> Result<&'a LoadedPlugin, OvertoneApiError> {
        if let Some(_v) = self.loaded_plugins.iter().find(|p| p.source.id == id) {
            return Err(OvertoneApiError::PluginAlreadyLoaded());
        }

        let plugins = match &self.file.plugins {
            None => return Err(OvertoneApiError::MissingPlugin(id)),
            Some(v) => v,
        };
        let plugin_ref = plugins.iter().find(|plug_ref| plug_ref.id == id);
        let plugin_ref = match plugin_ref {
            None => return Err(OvertoneApiError::MissingPlugin(id)),
            Some(p) => p,
        };

        let loaded: LoadedPlugin =
            LoadedPlugin::from_external_reference(&self.base_path, plugin_ref)?;

        let p_ref: &LoadedPlugin = self.loaded_plugins.push_and_get(loaded);

        return Ok(p_ref);
    }
}
