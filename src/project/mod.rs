use crate::plugin::{dependency::PluginDependencyEntry, errors::PluginError};
pub mod errors;
pub mod resource;
pub mod serialization;
pub mod arrangement;

use super::{editor::errors::OvertoneError, plugin::PluginBox};
use std::path::{Path, PathBuf};
use std::fs;
use serde_derive::{Deserialize, Serialize};
use arrangement::serialization::ArrangementHeader;
use crate::editor::errors::IOError;
use arrangement::errors::ArrangementError;
use arrangement::serialization::load_arrangement_from_directory;
use crate::utils::{Info};

/// Overtone Project, holds references to in-disk dependencies and manages
/// changes upon them (refactoring). It also contains loaded references to
/// the dependencies, and manages loading/unloading them.
#[derive(Debug)]
pub struct Project<'a> {
    /// The contents of the `Overtone.toml` manifest that marks
    /// a folder as an Overtone project.
    pub file: ProjectManifest,

    /// The path of the folder this project is saved to, if it is saved.
    pub directory: Option<PathBuf>,

    /// The plugins loaded into this project.
    /// Plugins are lazy-loaded, so this holds
    /// references to them when they load.
    pub loaded_plugins: Vec<PluginBox<'a>>,

    /// The "content" of a project.
    pub content: ProjectContent,
}

#[derive(Debug)]
/// Inside every project is a bunch of [`Arrangement`]s.
/// Here we only keep links to them — they are lazy loaded.
pub struct ProjectContent {
    pub arrangements: Vec<ArrangementHeader>,
}

impl<'a> Info for Project<'a> {
    fn get_name(&self) -> &str {
        self.file.info.name.as_str()
    }
}

impl<'a> Project<'a> {
    pub fn new(file: ProjectManifest) -> Self {
        Self {
            file,
            directory: None,
            loaded_plugins: Vec::new(),
            content: ProjectContent {
                arrangements: vec![],
            },
        }
    }

    /// Loads an overtone project from a directory, if there's a suitable manifest file.
    pub fn load_from_directory<S: Into<String>>(path: S) -> Result<Self, OvertoneError> {
        let path_str: String = path.into();
        let file = load_project_from_directory(&path_str)?;

        let dependencies = load_project_deps_from_directory(&path_str, &file.path_overrides)?;

        Ok(Project {
            file,
            directory: Some(PathBuf::from(path_str)),
            loaded_plugins: vec![],
            content: dependencies,
        })
    }

    pub fn get_plugins(&self) -> &Option<Vec<PluginDependencyEntry>> {
        &self.file.plugins
    }

    pub fn iter_loaded_plugins(&'a self) -> std::slice::Iter<'a, PluginBox<'a>> {
        self.loaded_plugins.iter()
    }

    /// Loads a plugin from a shared library located at the designated relative path.
    pub fn load_plugin(&'a mut self, plugin_id: String) -> Result<&'a PluginBox<'a>, PluginError> {
        if let Some(_v) = self.loaded_plugins.iter().find(|p| p.source.id == plugin_id) {
            return Err(PluginError::PluginAlreadyLoaded());
        }

        let plugins = match &self.file.plugins {
            None => return Err(PluginError::MissingPlugin(plugin_id)),
            Some(v) => v,
        };
        let plugin_ref = plugins.iter().find(|plug_ref| plug_ref.id == plugin_id);
        let plugin_ref = match plugin_ref {
            None => return Err(PluginError::MissingPlugin(plugin_id)),
            Some(p) => p,
        };

        let loaded: PluginBox = PluginBox::from_dependency_decl(&self.directory, plugin_ref)?;

        // TODO: Call the `on_plugin_load` callback passing a view to the project.
        //loaded.plugin.on_plugin_load(self);

        let loaded: &PluginBox = {
            self.loaded_plugins.push(loaded);
            self.loaded_plugins.last_mut().unwrap()
        };

        Ok(loaded)
    }
}

const OVERTONE_PROJECT_FILE_NAME: &str = "Overtone.toml";
const OVERTONE_ARRANGEMENTS_FOLDER_PATH: &str = "arrangements";

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectManifest {
    pub info: ProjectInfo,
    pub path_overrides: Option<ProjectPathOverrides>,
    pub plugins: Option<Vec<PluginDependencyEntry>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectInfo {
    pub name: String,
    pub authors: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectPathOverrides {
    arrangements_dir: Option<PathBuf>,
    default_export_dir: Option<PathBuf>,
}

pub fn load_project_file<P: AsRef<Path>>(path: P) -> Result<ProjectManifest, OvertoneError> {
    let proj_file_raw = match fs::read(path) {
        Err(e) => return Err(OvertoneError::GenericError(Some(e))),
        Ok(v) => match String::from_utf8(v) {
            Err(e) => return Err(OvertoneError::StringParsingError(e)),
            Ok(v) => v,
        },
    };
    let proj_file: Result<ProjectManifest, _> = toml::from_str(proj_file_raw.as_str());
    let proj_file = match proj_file {
        Err(e) => return Err(OvertoneError::TomlParsingError(e)),
        Ok(v) => v,
    };
    Ok(proj_file)
}

pub fn load_project_from_directory(path_str: &str) -> Result<ProjectManifest, OvertoneError> {
    let dir = match fs::read_dir(path_str) {
        Ok(v) => v,
        Err(e) => return Err(OvertoneError::IO(IOError::DirectoryNotFound(e))),
    };
    let dir_entry = dir.into_iter().find(|e| match e {
        Err(_) => false,
        Ok(v) => v.file_name() == OVERTONE_PROJECT_FILE_NAME,
    });
    let dir_entry = match dir_entry {
        None => {
            return Err(OvertoneError::IO(
                IOError::DirectoryIsNotOvertoneProject(None),
            ))
        }
        Some(v) => match v {
            Err(e) => {
                return Err(OvertoneError::IO(
                    IOError::DirectoryIsNotOvertoneProject(Some(e)),
                ))
            }
            Ok(v) => v,
        },
    };
    let file = load_project_file(dir_entry.path())?;
    Ok(file)
}

pub fn load_project_deps_from_directory(
    path_str: &String,
    path_overrides: &Option<ProjectPathOverrides>,
) -> Result<ProjectContent, OvertoneError> {
    let arrangements: Vec<ArrangementHeader> = load_project_arrangements(path_str, path_overrides)
        .map_err(OvertoneError::ArrangementError)?
        .into_iter()
        .collect::<Result<_, _>>()
        .map_err(OvertoneError::ArrangementError)?;

    Ok(ProjectContent { arrangements })
}

// TODO: This will be refactored out somewhere else.
fn load_project_arrangements(
    path_str: &String,
    path_overrides: &Option<ProjectPathOverrides>,
) -> Result<Vec<Result<ArrangementHeader, ArrangementError>>, ArrangementError> {
    let arrangements_dir_path = PathBuf::from(path_str).join(
        path_overrides
            .as_ref()
            .and_then(|po| po.arrangements_dir.as_ref())
            .unwrap_or(&PathBuf::from(OVERTONE_ARRANGEMENTS_FOLDER_PATH)),
    );

    let dir = match fs::read_dir(arrangements_dir_path.clone()) {
        Ok(v) => v,
        Err(e) => {
            fs::create_dir(arrangements_dir_path).map_err(ArrangementError::IOError)?;
            return Ok(vec![]);
        }
    };

    let arrangement_headers: Vec<Result<ArrangementHeader, ArrangementError>> = dir
        .into_iter()
        .filter_map(|entry| {
            let e = entry.map_err(ArrangementError::IOError);
            let e = match e {
                Ok(v) => v,
                Err(err) => return Some(Err(err)),
            };

            if e.path().is_dir() {
                Some(Ok(e))
            } else {
                None
            }
        })
        .map(|e| load_arrangement_from_directory(e?.path()))
        .collect();

    Ok(arrangement_headers)
}