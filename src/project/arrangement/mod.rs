use serde_derive::{Deserialize, Serialize};
use std::path::PathBuf;
use std::fs;
use crate::project::arrangement::dependency::ArrFragmentReference;
use crate::project::arrangement::errors::ArrangementError;
use crate::project::resource::{Resource, ResourceFieldError, ResourceFieldValue};

pub mod dependency;
pub mod errors;
pub mod time;

#[derive(Debug)]
pub struct Arrangement {
    pub header: ArrangementHeader,
}

const ARRANGEMENT_HEADER_FILE_NAME: &str = "header.toml";

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

impl ArrangementHeader {
    pub fn load_from_directory(
        path: PathBuf,
    ) -> Result<Self, ArrangementError> {
        // Check for the "index.toml" file inside.
        let header_path = path.join(ARRANGEMENT_HEADER_FILE_NAME);
        let header_bytes = fs::read(header_path).map_err(ArrangementError::HeaderIOError)?;
        let header_raw =
            String::from_utf8(header_bytes).map_err(ArrangementError::HeaderStringError)?;
        let header: Self =
            toml::from_str(&header_raw).map_err(ArrangementError::HeaderFormatError)?;

        Ok(header)
    }
}

impl<'a> Resource<'a> for Arrangement {
    fn get_field_ids() -> &'a [&'static str] {
        &["name"]
    }

    fn get_field_value(
        &self,
        field_id: &'static str,
    ) -> Result<ResourceFieldValue, ResourceFieldError> {
        Err(ResourceFieldError::FieldDoesntExist)
    }

    fn set_field_value(
        &mut self,
        field_id: &'static str,
        value: ResourceFieldValue,
    ) -> Result<(), ResourceFieldError> {
        match field_id {
            "name" => match value {
                ResourceFieldValue::Text(t) => self.header.meta.name = t.to_string(),
                _ => return Err(ResourceFieldError::UnacceptableValue),
            },
            _ => return Err(ResourceFieldError::FieldDoesntExist),
        }

        Ok(())
    }

    fn save() -> Result<(), crate::project::resource::ResourceSaveError> {
        Ok(())
    }
}
