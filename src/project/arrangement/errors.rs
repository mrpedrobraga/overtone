use std::string::FromUtf8Error;

use crate::editor::errors::OvertoneError;

#[derive(Debug)]
pub enum ArrangementError {
    MissingFolder,
    IOError(std::io::Error),

    HeaderIOError(std::io::Error),
    HeaderStringError(FromUtf8Error),
    HeaderFormatError(toml::de::Error),
}

impl From<ArrangementError> for OvertoneError {
    fn from(value: ArrangementError) -> Self {
        OvertoneError::ArrangementError(value)
    }
}