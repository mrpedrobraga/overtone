use crate::serialization::project::{load_project_file, ProjectFile};

use super::{errors::OvertoneApiError, info::Info, plugin::LoadedPlugin, utils::PushReturn};
use std::fs;

#[derive(Debug)]
pub struct Project<'a> {
    pub file: ProjectFile,
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
            loaded_plugins: Vec::new(),
        }
    }

    // Loads an overtone project from a directory, looking for an `Overtone.toml` file.
    pub fn load_from_directory<S: Into<String>>(path: S) -> Result<Self, OvertoneApiError> {
        let dir = match fs::read_dir(path.into()) {
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
            loaded_plugins: vec![],
        })
    }

    // Loads a plugin from a library located at its path.
    pub fn load_plugin(&'a mut self, id: &'static str) -> Result<&LoadedPlugin, OvertoneApiError> {
        let plugins = match &self.file.plugins {
            None => return Err(OvertoneApiError::MissingPlugin(id)),
            Some(v) => v,
        };
        let plugin_ref = plugins.iter().find(|plug_ref| plug_ref.id == id);
        let plugin_ref = match plugin_ref {
            None => return Err(OvertoneApiError::MissingPlugin(id)),
            Some(p) => p,
        };

        let loaded = LoadedPlugin::from_external_reference(plugin_ref)?;

        let p_ref = self.loaded_plugins.push_and_get(loaded);

        return Ok(p_ref);
    }
}
