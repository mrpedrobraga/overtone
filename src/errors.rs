//! All the API errors.

use std::string::FromUtf8Error;

#[derive(Debug)]
pub enum OvertoneApiError {
    // A generic error. This is a code smell and will be removed from Overtone as stability grows.
    GenericError(Option<std::io::Error>),

    IO(IOError),

    ArrangementIOError(std::io::Error),

    TomlParsingError(toml::de::Error),
    StringParsingError(FromUtf8Error),

    PluginAlreadyLoaded(),
    MissingPlugin(String),
    LibraryNotFound(libloading::Error),
    LibraryIsNotOvertonePlugin(),
}

#[derive(Debug)]
pub enum IOError {
    DirectoryNotFound(std::io::Error),
    FileNotFound(std::io::Error),
    DirectoryIsNotOvertoneProject(Option<std::io::Error>),
    ErrorOpeningProject(std::io::Error),
}
