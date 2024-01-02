#[derive(Debug)]
pub enum ArrangementError {
    MissingFolder,
    IOError(std::io::Error),
}
