use std::{fs, path::Path};

use serde_derive::{Deserialize, Serialize};

use crate::errors::OvertoneApiError;

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
