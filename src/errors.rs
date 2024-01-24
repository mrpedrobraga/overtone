//! # API Errors
//!
//! Errors in the Overtone API are detailed trees, which precisely map the source of a failure.
//! This design allows the API (and the frontend) to fail gracefully and avoid crashes.
//!
//! From dependencies, to memory errors, the API should avoid crashing as much as possible,
//! and inform the consumer on how to act to recover from the failure.

use std::string::FromUtf8Error;

use crate::{
    arrangement::errors::ArrangementError, plugin::errors::PluginError,
    project::errors::ProjectError,
};

#[derive(Debug)]
pub enum OvertoneApiError {
    // A generic error. This is a code smell and will be removed from Overtone as stability grows.
    GenericError(Option<std::io::Error>),

    TomlParsingError(toml::de::Error),
    StringParsingError(FromUtf8Error),

    IO(IOError),

    ProjectError(ProjectError),
    ArrangementError(ArrangementError),
    PluginError(PluginError),
}

#[derive(Debug)]
pub enum IOError {
    DirectoryNotFound(std::io::Error),
    FileNotFound(std::io::Error),
    DirectoryIsNotOvertoneProject(Option<std::io::Error>),
    ErrorOpeningProject(std::io::Error),
}
