use crate::editor::errors::OvertoneError;

#[derive(Debug)]
pub enum ProjectError {}

impl From<ProjectError> for OvertoneError {
    fn from(value: ProjectError) -> Self {
        OvertoneError::ProjectError(value)
    }
}