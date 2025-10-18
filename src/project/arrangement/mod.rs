//! # Arrangements
//!
//! The soul of an Overtone project is the arrangements.
//!
//! Arrangements combine [`Fragment`]s together to create things (i.e. songs, video).
//! Fragments themselves might be internally composed of sub-elements in a graph-like fashion.
//!
//! > This is a bit of an esoteric way of putting things, but the generality of this concept
//! > is why Overtone is so powerful.
//! >
//! > As an example, a Fragment of a song can be something like a single audio sample or a Piano Roll
//! > as it shows in the track editor. Internally, the individual notes of a piano roll are considered
//! > its elements, etc.
//!
//! An arrangement contains a single fragment, which seems to imply you can only have One Thing
//! in your song, but you can choose it to be a kind of fragment that itself can house many elements
//! like a Piano Roll, a Multi Type Fragment, etc.;
//!
//! ## Lazy Loading
//!
//! Fragments are lazy-loaded, this is so you can navigate through hundreds of thousands
//! of arrangements seamlessly. Furthermore, some Fragments are "owned" by the [`Project`]
//! allowing you to use it in many different arrangements.
//!
//! ## Serialization
//!
//! An arrangement is saved on disk as a folder, which allows you to see all of its parts.

use serde_derive::{Deserialize, Serialize};
use std::path::PathBuf;
use std::fs;
use std::string::FromUtf8Error;
use crate::{DependencyId, OvertoneError};
use crate::project::resource::{Resource, ResourceFieldInfo, ResourceFieldValue, ResourceGetFieldError, ResourceSetFieldError};

pub mod elements;
pub mod time;

const ARRANGEMENT_HEADER_FILE_NAME: &str = "header.toml";

#[derive(Serialize, Deserialize, Debug)]
/// An arrangement.
pub struct Arrangement {
    pub meta: ArrangementMetadata,
    pub content: ArrangementContent,
}

#[derive(Serialize, Deserialize, Debug)]
/// Basic information about an Arrangement.
/// > This can optionally be written to the files you export (like MP3 ID3).
pub struct ArrangementMetadata {
    pub name: String,
    pub authors: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
/// The content of an arrangement — as of now it's just a singular
/// reference to an arrangement fragment.
pub struct ArrangementContent {
    root_fragment: ArrFragmentReference,
}

#[derive(Serialize, Deserialize, Debug)]
/// A reference to an arrangement fragment...
///
/// TODO: Maybe formalise a "dependency system" for these things.
pub struct ArrFragmentReference {
    pub id: DependencyId,
}

impl Arrangement {
    /// Loads an arrangement from a directory containing a `header.toml` file.
    pub fn load_from_directory(
        path: PathBuf,
    ) -> Result<Self, ArrangementError> {
        // Check for the header file inside.
        let header_path = path.join(ARRANGEMENT_HEADER_FILE_NAME);
        let header_bytes = fs::read(header_path).map_err(ArrangementError::HeaderIOError)?;
        let header_raw =
            String::from_utf8(header_bytes).map_err(ArrangementError::HeaderEncodingError)?;
        let header: Self =
            toml::from_str(&header_raw).map_err(ArrangementError::HeaderDeserializeError)?;

        Ok(header)
    }
}

impl<'a> Resource<'a> for Arrangement {
    fn get_fields_info() -> &'a [ResourceFieldInfo] {
        &[
            ResourceFieldInfo { name: "name" }
        ]
    }

    fn get_field_value(
        &self,
        field_id: &'static str,
    ) -> Result<ResourceFieldValue, ResourceGetFieldError> {
        Err(ResourceGetFieldError::NoSuchField)
    }

    fn set_field_value(
        &mut self,
        field_id: &'static str,
        value: ResourceFieldValue,
    ) -> Result<(), ResourceSetFieldError> {
        match field_id {
            "name" => match value {
                ResourceFieldValue::Text(t) => self.meta.name = t.to_string(),
                _ => return Err(ResourceSetFieldError::IncompatibleType),
            },
            _ => return Err(ResourceSetFieldError::NoSuchField),
        }

        Ok(())
    }

    fn save() -> Result<(), crate::project::resource::ResourceSaveError> {
        Ok(())
    }
}

// MARK: Errors

#[derive(Debug)]
/// An error that originated from doing something regarding [`Arrangement`]s.
pub enum ArrangementError {
    /// The folder the arrangement was supposed to be contained in does not exist.
    MissingFolder,
    /// An error occurred when doing IO.
    IOError(std::io::Error),
    /// An error occurred when trying to do IO with the header.
    HeaderIOError(std::io::Error),
    /// An error occurred when trying to parse the header as UTF-8.
    HeaderEncodingError(FromUtf8Error),
    /// An error occurred when deserializing the header, probably because it's malformed.
    HeaderDeserializeError(toml::de::Error),
}

impl From<ArrangementError> for OvertoneError {
    fn from(value: ArrangementError) -> Self {
        OvertoneError::ArrangementError(value)
    }
}
