use std::string::FromUtf8Error;

#[derive(Debug)]
pub enum OvertoneApiError {
    GenericError(Option<std::io::Error>),
    DirectoryNotFound(std::io::Error),
    FileNotFound(std::io::Error),

    DirectoryIsNotOvertoneProject(Option<std::io::Error>),
    ErrorOpeningProject(std::io::Error),

    TomlParsingError(toml::de::Error),
    StringParsingError(FromUtf8Error),

    MissingPlugin(&'static str),
    LibraryNotFound(libloading::Error),
    LibraryIsNotOvertonePlugin(),
}
