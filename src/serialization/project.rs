use crate::errors::OvertoneApiError;
use serde_derive::{Deserialize, Serialize};
use std::{fs, path::Path};

const OVERTONE_PROJECT_FILE_NAME: &'static str = "Overtone.toml";

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

#[derive(Serialize, Deserialize, Debug)]
pub struct PluginDependencyEntry {
    pub id: String,
    pub path: String,
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
    Ok(file)
}
