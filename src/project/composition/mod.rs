//! # Arrangements
//!
//! The soul of an Overtone project is the compositions.
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
//! An composition contains a single fragment, which seems to imply you can only have One Thing
//! in your song, but you can choose it to be a kind of fragment that itself can house many elements
//! like a Piano Roll, a Multi Type Fragment, etc.;
//!
//! ## Lazy Loading
//!
//! Fragments are lazy-loaded, this is so you can navigate through hundreds of thousands
//! of compositions seamlessly. Furthermore, some Fragments are "owned" by the [`Project`]
//! allowing you to use it in many different compositions.
//!
//! ## Serialization
//!
//! An composition is saved on disk as a folder, which allows you to see all of its parts.

use serde_derive::{Deserialize, Serialize};
use std::path::PathBuf;
use std::fs;
use std::string::FromUtf8Error;
use crate::{DependencyId, OvertoneError};
use crate::project::resource::{Resource, ResourceFieldInfo, ResourceFieldValue, ResourceGetFieldError, ResourceSetFieldError};

pub mod elements;
pub mod time;

const COMPOSITION_HEADER_FILENAME: &str = "header.toml";

#[derive(Serialize, Deserialize, Debug)]
pub struct Composition {
    pub meta: CompositionMetadata,
    pub content: CompositionContent,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CompositionMetadata {
    pub name: String,
    pub authors: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CompositionContent {
    root_fragment: ArrFragmentReference,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ArrFragmentReference {
    /// TODO: This will be replaced by `ResourceId` in alpha.
    #[deprecated]
    pub id: DependencyId,
}

impl Composition {
    /// Loads an composition from a directory containing a `header.toml` file.
    pub fn load_from_directory(
        path: PathBuf,
    ) -> Result<Self, CompositionError> {
        // Check for the header file inside.
        let header_path = path.join(COMPOSITION_HEADER_FILENAME);
        let header_bytes = fs::read(header_path).map_err(CompositionError::HeaderIOError)?;
        let header_raw =
            String::from_utf8(header_bytes).map_err(CompositionError::HeaderEncodingError)?;
        let header: Self =
            toml::from_str(&header_raw).map_err(CompositionError::HeaderDeserializeError)?;

        Ok(header)
    }
}

impl<'a> Resource<'a> for Composition {
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
/// An error that originated from doing something regarding [`Composition`]s.
pub enum CompositionError {
    /// The folder the composition was supposed to be contained in does not exist.
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

impl From<CompositionError> for OvertoneError {
    fn from(value: CompositionError) -> Self {
        OvertoneError::CompositionError(value)
    }
}
