//! # Project
//!
//! An Overtone [`Project`] is a directory containing arrangements,
//! plugins, resources, etc.
//!
//! On disk, an Overtone project is recognised as any directory containing a valid
//! `Overtone.toml` manifest.
//!
//! The contents of a project are lazy-loaded to save on memory. Arrangements, plugins
//! and other resources are only loaded when needed and can be unloaded when no longer in use.
//! This can save a ton of memory, though, it puts some constraints on how to write code
//! for this library. You see, the contents of a project are separate files that will be
//! interwoven in references and dependencies that shan't be broken.
//!
//! ## Editing a Project
//!
//! To maintain the invariants of a project intact, a project should be edited through
//! [`super::editor`].

use std::collections::HashMap;
use crate::plugin::{PluginDependencyEntry, PluginError};
pub mod arrangement;
pub mod resource;

use super::{plugin::LoadedPlugin, Info, OvertoneError};
use crate::project::arrangement::Arrangement;
use crate::IOError;
use arrangement::ArrangementError;
use serde_derive::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

/// An Overtone project.
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
    pub loaded_plugins: Vec<LoadedPlugin<'a>>,

    /// The "content" of a project.
    pub content: ProjectContent,
}

impl<'a> Info for Project<'a> {
    fn get_name(&self) -> &str {
        self.file.info.name.as_str()
    }
}

#[derive(Debug)]
/// Inside every project is a bunch of [`Arrangement`]s.
/// Here we only keep links to them — they are lazy loaded.
pub struct ProjectContent {
    pub arrangements: Vec<Arrangement>,
}

const OVERTONE_PROJECT_FILE_NAME: &str = "Overtone.toml";
const DEFAULT_ARRANGEMENTS_DIRECTORY_PATH: &str = "arrangements";

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectManifest {
    pub info: ProjectInfo,
    #[serde(default)]
    pub configuration_overrides: ConfigurationOverrides,
    pub plugins: HashMap<String, PluginDependencyEntry>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectInfo {
    pub name: String,
    pub authors: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ConfigurationOverrides {
    arrangements_dir: Option<PathBuf>,
    default_export_dir: Option<PathBuf>,
}

impl ProjectManifest {
    /// Loads a project manifest from a path to the `Overtone.toml` file.
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self, OvertoneError> {
        let manifest_file_raw = fs::read(path).map_err(|e| OvertoneError::GenericError(Some(e)))?;
        let manifest_str =
            String::from_utf8(manifest_file_raw).map_err(OvertoneError::StringParsingError)?;
        toml::from_str(&manifest_str).map_err(OvertoneError::TomlDeserializingError)
    }

    pub fn save_to_path<P: AsRef<Path>>(&self, path: P) -> Result<(), OvertoneError> {
        fs::write(
            path,
            toml::to_string_pretty(self).map_err(OvertoneError::TomlSerializingError)?,
        )
        .map_err(IOError::Generic)?;

        Ok(())
    }

    /// Loads a project manifest from a path to a directory that contains an `Overtone.toml` file.
    pub fn load_from_directory<P: AsRef<Path>>(path: P) -> Result<Self, OvertoneError> {
        let dir =
            fs::read_dir(path).map_err(|e| OvertoneError::IO(IOError::DirectoryNotFound(e)))?;

        let dir_entry = dir
            .filter_map(Result::ok)
            .find(|v| v.file_name() == OVERTONE_PROJECT_FILE_NAME)
            .ok_or_else(|| OvertoneError::IO(IOError::DirectoryIsNotOvertoneProject(None)))?;

        Self::load_from_file(dir_entry.path())
    }
}

impl<'a> Project<'a> {
    /// Creates a new project with configuration.
    ///
    /// This function takes a [`ProjectManifest`] but don't get confused,
    /// no `Overtone.toml` file will exist on disk at this point. If anything,
    /// the manifest will be reified whenever the project is saved to disk.
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
    pub fn load_from_directory<P: AsRef<Path>>(path: P) -> Result<Self, OvertoneError> {
        let file = ProjectManifest::load_from_directory(&path)?;

        let content = ProjectContent::load_from_directory(&path, &file.configuration_overrides)?;

        Ok(Project {
            file,
            directory: Some(PathBuf::from(path.as_ref())),
            loaded_plugins: vec![],
            content,
        })
    }

    /// Saves the project to the given directory.
    ///
    /// If the path given here is `/home/user/Music/`, and the project is named "Game OST",
    /// it will create a `/home/user/Music/Game OST/Overtone.toml`.
    pub fn save_to_new_directory<P: AsRef<Path>>(&self, path: P) -> Result<(), OvertoneError> {
        self.initialize_directory(&path)?;
        self.file.save_to_path(
            path.as_ref()
                .join(self.file.info.name.as_str())
                .join(OVERTONE_PROJECT_FILE_NAME),
        )?;
        Ok(())
    }

    pub fn initialize_directory<P: AsRef<Path>>(&self, path: P) -> Result<(), OvertoneError> {
        let path = path.as_ref();

        if !path.is_dir() {
            return Err(OvertoneError::ProjectError(
                ProjectError::SaveLocationNotADirectory,
            ));
        }

        let path = path.join(self.file.info.name.as_str());
        let path: &Path = path.as_ref();

        if path.exists() {
            return Err(OvertoneError::ProjectError(
                ProjectError::SaveLocationAlreadyExists,
            ));
        }

        std::fs::create_dir(&path).map_err(IOError::Generic)?;
        std::fs::create_dir(&path.join("assets")).map_err(IOError::Generic)?;
        std::fs::create_dir(&path.join("arrangements")).map_err(IOError::Generic)?;
        std::fs::create_dir(&path.join("exports")).map_err(IOError::Generic)?;

        Ok(())
    }

    /// Quick way of retrieving a project's plugins.
    pub fn get_plugins(&self) -> &HashMap<String, PluginDependencyEntry> {
        &self.file.plugins
    }

    /// Returns an iterators through the loaded plugins. Might be useful.
    pub fn iter_loaded_plugins(&'a self) -> std::slice::Iter<'a, LoadedPlugin<'a>> {
        self.loaded_plugins.iter()
    }

    /// Loads a plugin given its id. The plugin in question must have been "installed," that is,
    /// have a dependency entry in the project containing the path of the shared library.
    ///
    /// This function also conveniently returns a reference to the [`LoadedPlugin`].
    pub fn load_plugin(
        &'a mut self,
        plugin_id: String,
    ) -> Result<&'a LoadedPlugin<'a>, PluginError> {
        if self.loaded_plugins.iter().any(|p| p.id == plugin_id) {
            return Err(PluginError::PluginAlreadyLoaded());
        }

        let entry = self
            .file
            .plugins
            .iter()
            .find(|p| p.0.as_str().eq(&plugin_id))
            .ok_or_else(|| PluginError::MissingPlugin(plugin_id.clone()))?;

        let loaded = LoadedPlugin::load_from_dependency_entry(&self.directory, entry.0, entry.1)?;

        self.loaded_plugins.push(loaded);
        Ok(self.loaded_plugins.last().unwrap())
    }
}

impl ProjectContent {
    /// Fetches the project's contents from disk.
    ///
    /// This function only loads the headers and metadata and does not actually load everything
    /// into memory :-)
    pub fn load_from_directory<P: AsRef<Path>>(
        path_str: P,
        path_overrides: &ConfigurationOverrides,
    ) -> Result<Self, OvertoneError> {
        let arrangements: Vec<Arrangement> = load_project_arrangements(path_str, path_overrides)
            .map_err(OvertoneError::ArrangementError)?
            .into_iter()
            .collect::<Result<_, _>>()
            .map_err(OvertoneError::ArrangementError)?;

        Ok(Self { arrangements })
    }
}

// TODO: This will be refactored out somewhere else.
fn load_project_arrangements<P: AsRef<Path>>(
    path: P,
    path_overrides: &ConfigurationOverrides,
) -> Result<Vec<Result<Arrangement, ArrangementError>>, ArrangementError> {
    let default_arrangements_directory_path: &Path = Path::new(DEFAULT_ARRANGEMENTS_DIRECTORY_PATH);
    let dir_path = if let Some(path_buf) = &path_overrides.arrangements_dir {
        path_buf.as_path()
    } else {
        default_arrangements_directory_path
    };

    let dir_path = path.as_ref().join(&dir_path);

    // This will read a directory if it exists or create it if it doesn't.
    let dir = fs::read_dir(&dir_path).or_else(|e| {
        fs::create_dir_all(&dir_path).map_err(ArrangementError::IOError)?;
        Ok(fs::read_dir(&dir_path).map_err(ArrangementError::IOError)?)
    })?;

    let headers = dir
        .filter_map(|entry| {
            let entry = entry.ok()?;
            entry
                .path()
                .is_dir()
                .then(|| Arrangement::load_from_directory(entry.path()))
        })
        .collect();

    Ok(headers)
}
// MARK: Errors

#[derive(Debug)]
pub enum ProjectError {
    SaveLocationAlreadyExists,
    SaveLocationNotADirectory,
}

impl From<ProjectError> for OvertoneError {
    fn from(value: ProjectError) -> Self {
        OvertoneError::ProjectError(value)
    }
}
