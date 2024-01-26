use crate::errors::OvertoneApiError;

#[derive(Debug)]
pub enum ProjectError {}

impl From<ProjectError> for OvertoneApiError {
    fn from(value: ProjectError) -> Self {
        OvertoneApiError::ProjectError(value)
    }
}