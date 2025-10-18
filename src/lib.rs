//! # Overtone
//!
//! An API for management of musical projects,
//! that handles files, dependencies, plugins and actions,
//! while keeping the parts generic enough that you can
//! use them to do anything.
//!
//! ## Project
//!
//! An Overtone project is a folder which has an `Overtone.toml`
//! manifest.
//!
//! Here's a quick example of opening a project:
//! ```no_run
//! # use overtone::{ project::Project, errors::OvertoneApiError };
//! let mut p = Project::load_from_directory("./examples/test_project")?;
//! # Ok::<(), OvertoneApiError>(())
//! ```
//!
//! With that, you can modify the project in memory, until it's time to save it.
//!
//! Needless to say, editing programs using the API is cumbersome and using the GUI is better.

#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unreachable_code)]

pub mod editor;
pub mod plugin;
pub mod project;
pub mod renderer;
pub mod transformer;
pub mod task;

/// Trait that allows extracting some metadata from foreign types.
pub trait Info {
    fn get_name(&self) -> &str;
}

pub type RefStr = std::rc::Rc<str>;
pub type DependencyId = String;

#[derive(Debug)]
pub enum OvertoneError {
    /// A generic error. This is a code smell and will be removed from Overtone as stability grows.
    GenericError(Option<std::io::Error>),

    TomlDeserializingError(toml::de::Error),
    TomlSerializingError(toml::ser::Error),
    StringParsingError(std::string::FromUtf8Error),

    IO(IOError),

    ProjectError(project::ProjectError),
    ArrangementError(project::arrangement::ArrangementError),
    PluginError(crate::plugin::PluginError),
}

#[derive(Debug)]
pub enum IOError {
    /// Another code smell.
    ErrorOpeningProject(std::io::Error),
    DirectoryNotFound(std::io::Error),
    FileNotFound(std::io::Error),
    DirectoryIsNotOvertoneProject(Option<std::io::Error>),
    Generic(std::io::Error),
}

impl From<IOError> for OvertoneError {
    fn from(value: IOError) -> Self {
        OvertoneError::IO(value)
    }
}
