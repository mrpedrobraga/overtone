//! # Editors
//!
//! Editors hold projects and allow clients to modify them.

use crate::project::Project;
use crate::resource::{ResourceId, ResourceProvider, ResourceProviderHeader};

/// An editor which allows editing of databases of Resources
/// through an API.
pub trait Editor {
    /// The type of Project this editor edits.
    type Project: Project;
    /// The type of Client that you can ask of this editor.
    type Client<'editor>: EditorClient where Self: 'editor;

    /// Returns a new client that can edit things.
    ///
    /// I can maybe ask for a "ClientSetup" parameter
    /// here if clients need anything to be created.
    fn new_client<'editor>(&'editor self) -> Self::Client<'editor>;
}

/// A trait for a client that can use an editor to edit things.
pub trait EditorClient {
    /// Returns information about all the resource providers the project has.
    fn list_resource_providers(&self) -> impl Iterator<Item = ResourceProviderHeader>;

    /// Returns all the resource headers for a bank.
    ///
    /// TODO: Probably return a virtualized list (impl `Paginator`) that we can scroll?
    /// For now it should maybe just return an iterator.
    fn list_resources(&self, bank: Option<()>, search_term: Option<&str>);

    /// Returns a Resource given its id.
    fn get_resource(&self, id: ResourceId) -> Option<()>;

    /// Previews the result of an action.
    fn preview(&self, action: &Action);

    /// Commits an action.
    fn commit(&mut self, action: Action);
}

pub enum Action {
    EditField {
        rid: (),
        field: (),
        value: Box<dyn Value>,
    },
    InvokeMethod {
        rid: (),
        method: (),
        args: Vec<Box<dyn Value>>,
    },
    Global(String),
}

/// Trait for anything that might want to be set to a resource over the wire.
pub trait Value {}
