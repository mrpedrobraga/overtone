use std::string::FromUtf8Error;

#[derive(Debug)]
pub enum ArrangementError {
    MissingFolder,
    IOError(std::io::Error),

    HeaderIOError(std::io::Error),
    HeaderStringError(FromUtf8Error),
    HeaderFormatError(toml::de::Error),
}
