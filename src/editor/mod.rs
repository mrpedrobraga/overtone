//! # The Editor API
//!
//! This module has types, functions and utilities for editing an Overtone
//! project while keeping invariants intact.
//!
//! The GUI uses this module to edit the project, too.

use crate::OvertoneError;
use crate::project::Project;

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

/// Identification of the Client associated with a given action.
pub struct Client<'a> {
    id: &'a str
}

/// Describes an action the editor can perform.
pub struct Action {}

impl Action {}