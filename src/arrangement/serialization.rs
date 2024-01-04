use std::{fs, path::PathBuf};

use super::{dependency::ArrFragmentReference, errors::ArrangementError};
use serde_derive::{Deserialize, Serialize};

const ARRANGEMENT_HEADER_FILE_NAME: &'static str = "header.toml";

#[derive(Serialize, Deserialize, Debug)]
pub struct ArrangementHeader {
    pub meta: ArrangementHeaderInfo,
    pub editor: ArrangementHeaderEditorInfo,
    pub content: ArrangementHeaderContent,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ArrangementHeaderInfo {
    pub name: String,
    pub authors: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ArrangementHeaderEditorInfo {
    pub requires_version: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ArrangementHeaderContent {
    root_fragment: ArrFragmentReference,
}

pub fn load_arrangement_from_directory(
    path: PathBuf,
) -> Result<ArrangementHeader, ArrangementError> {
    // Check for the "index.toml" file inside.
    let header_path = path.join(ARRANGEMENT_HEADER_FILE_NAME);
    let header_bytes = fs::read(header_path).map_err(|e| ArrangementError::HeaderIOError(e))?;
    let header_raw =
        String::from_utf8(header_bytes).map_err(|e| ArrangementError::HeaderStringError(e))?;
    let header: ArrangementHeader =
        toml::from_str(&header_raw).map_err(|e| ArrangementError::HeaderFormatError(e))?;

    Ok(header)
}
