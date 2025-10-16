use crate::editor::errors::OvertoneError;
use crate::project::Project;

pub mod errors;

/// Edits an overtone Project
pub struct Editor<'a> {
    project: Project<'a>,
}

impl<'a> Editor<'a> {
    fn new(project: Project<'a>) -> Self {
        Editor {
            project
        }
    }

    fn do_action(&mut self, client: &Client<'a>, action: Action) -> Result<(), OvertoneError> {
        todo!()
    }
}

/// A client in identifier
pub struct Client<'a> {
    id: &'a str
}

/// An action which can be consumed by an editor.
pub struct Action {}

impl Action {}