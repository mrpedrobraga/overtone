use super::{
    errors::OvertoneApiError,
    info::Info,
    plugin::{ExternalPluginReference, LoadedPlugin},
    utils::PushReturn,
};
use serde_derive::{Deserialize, Serialize};
use std::fs;

/// An Overtone project, which refers to (and can load) plugins, files, arrangements.
#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectFile {
    pub info: ProjectInfo,
    pub plugins: Option<Vec<ExternalPluginReference>>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectInfo {
    pub name: String,
    pub authors: Option<Vec<String>>,
}

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
        Project {
            file,
            loaded_plugins: Vec::new(),
        }
    }

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
        let proj_file_raw = match fs::read(dir_entry.path()) {
            Err(e) => return Err(OvertoneApiError::GenericError(Some(e))),
            Ok(v) => match String::from_utf8(v) {
                Err(e) => return Err(OvertoneApiError::StringParsingError(e)),
                Ok(v) => v,
            },
        };
        let proj_file: Result<ProjectFile, _> = toml::from_str(proj_file_raw.as_str());
        let proj_file = match proj_file {
            Err(e) => return Err(OvertoneApiError::TomlParsingError(e)),
            Ok(v) => v,
        };

        Ok(Project::new(proj_file))
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
