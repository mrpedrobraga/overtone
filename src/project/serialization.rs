use crate::{
    api::errors::{IOError, OvertoneApiError},
    arrangement::{
        errors::ArrangementError,
        serialization::{load_arrangement_from_directory, ArrangementHeader},
    },
    plugin::dependency::PluginDependencyEntry,
    project::ProjectDependencies,
};
use serde_derive::{Deserialize, Serialize};
use std::{
    fs,
    path::{Path, PathBuf},
};

const OVERTONE_PROJECT_FILE_NAME: &str = "Overtone.toml";
const OVERTONE_ARRANGEMENTS_FOLDER_PATH: &str = "arrangements";

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectFile {
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

pub fn load_project_file<P: AsRef<Path>>(path: P) -> Result<ProjectFile, OvertoneApiError> {
    let proj_file_raw = match fs::read(path) {
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
    Ok(proj_file)
}

pub fn load_project_from_directory(path_str: &str) -> Result<ProjectFile, OvertoneApiError> {
    let dir = match fs::read_dir(path_str) {
        Ok(v) => v,
        Err(e) => return Err(OvertoneApiError::IO(IOError::DirectoryNotFound(e))),
    };
    let dir_entry = dir.into_iter().find(|e| match e {
        Err(_) => false,
        Ok(v) => v.file_name() == OVERTONE_PROJECT_FILE_NAME,
    });
    let dir_entry = match dir_entry {
        None => {
            return Err(OvertoneApiError::IO(
                IOError::DirectoryIsNotOvertoneProject(None),
            ))
        }
        Some(v) => match v {
            Err(e) => {
                return Err(OvertoneApiError::IO(
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
) -> Result<ProjectDependencies, OvertoneApiError> {
    let arrangements: Vec<ArrangementHeader> = load_project_arrangements(path_str, path_overrides)
        .map_err(OvertoneApiError::ArrangementError)?
        .into_iter()
        .collect::<Result<_, _>>()
        .map_err(OvertoneApiError::ArrangementError)?;

    Ok(ProjectDependencies { arrangements })
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
