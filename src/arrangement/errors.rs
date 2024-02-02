use std::string::FromUtf8Error;

use crate::api::errors::OvertoneApiError;

#[derive(Debug)]
pub enum ArrangementError {
    MissingFolder,
    IOError(std::io::Error),

    HeaderIOError(std::io::Error),
    HeaderStringError(FromUtf8Error),
    HeaderFormatError(toml::de::Error),
}

impl From<ArrangementError> for OvertoneApiError {
    fn from(value: ArrangementError) -> Self {
        OvertoneApiError::ArrangementError(value)
    }
}