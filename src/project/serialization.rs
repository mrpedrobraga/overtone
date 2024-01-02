use crate::{
    arrangement::{
        errors::ArrangementError,
        serialization::{ArrangementHeader, ArrangementHeaderInfo},
    },
    errors::{IOError, OvertoneApiError},
    project::ProjectDependencies,
    serialization::dependency::PluginDependencyEntry,
};
use serde_derive::{Deserialize, Serialize};
use std::{
    fs,
    path::{Path, PathBuf},
};

const OVERTONE_PROJECT_FILE_NAME: &'static str = "Overtone.toml";

const OVERTONE_ARRANGEMENTS_FOLDER_PATH: &'static str = "arrangements";

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectFile {
    pub info: ProjectInfo,
    pub plugins: Option<Vec<PluginDependencyEntry>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectInfo {
    pub name: String,
    pub authors: Option<Vec<String>>,
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

pub fn load_project_from_directory(path_str: &String) -> Result<ProjectFile, OvertoneApiError> {
    let dir = match fs::read_dir(path_str.clone()) {
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
) -> Result<ProjectDependencies, OvertoneApiError> {
    let arrangements: Vec<ArrangementHeader> = load_project_arrangements(path_str)?
        .into_iter()
        .collect::<Result<_, _>>()?;

    Ok(ProjectDependencies { arrangements })
}

// TODO: This will be refactored out somewhere else.
fn load_project_arrangements(
    path_str: &String,
) -> Result<Vec<Result<ArrangementHeader, OvertoneApiError>>, OvertoneApiError> {
    let arrangements_dir_path = PathBuf::from(path_str).join(OVERTONE_ARRANGEMENTS_FOLDER_PATH);

    let dir = match fs::read_dir(arrangements_dir_path.clone()) {
        Ok(v) => v,
        Err(e) => {
            fs::create_dir(arrangements_dir_path)
                .map_err(|e| OvertoneApiError::ArrangementError(ArrangementError::IOError(e)))?;
            return Ok(vec![]);
        }
    };

    let arrangement_headers: Vec<Result<ArrangementHeader, OvertoneApiError>> = dir
        .into_iter()
        .filter_map(|entry| {
            let e =
                entry.map_err(|e| OvertoneApiError::ArrangementError(ArrangementError::IOError(e)));
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
        .map(|entry| {
            let e = entry?;

            // Check for the "index.toml" file inside.

            Ok(ArrangementHeader {
                info: ArrangementHeaderInfo {
                    name: e.path().to_str().unwrap_or("Unknown").to_string(),
                },
            })
        })
        .collect();

    Ok(vec![])
}
