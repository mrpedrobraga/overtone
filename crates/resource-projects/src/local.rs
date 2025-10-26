use std::iter::empty;
use crate::editor::Editor;
use crate::editor::{Action, EditorClient};
use crate::project::Project;
use crate::resource::{ResourceId, ResourceProvider, ResourceProviderHeader};

pub struct LocalEditor<T> {
    instance: T
}
impl <T> LocalEditor<T> {
    pub fn new(instance: T) -> Self {
        Self {
            instance
        }
    }
}

impl<T: Project> Editor for LocalEditor<T> {
    type Project = T;
    type Client<'editor> = LocalClient<'editor, T> where T: 'editor;

    fn new_client<'editor>(&'editor self) -> Self::Client<'editor> {
        LocalClient {
            editor: &self
        }
    }
}

pub struct LocalClient<'editor, T> {
    editor: &'editor LocalEditor<T>
}

impl<'editor, T: Project> EditorClient for LocalClient<'editor, T> {
    fn list_resource_providers(&self) -> impl Iterator<Item = ResourceProviderHeader> {
        self.editor.instance.list_resource_providers()
    }

    fn list_resources(&self, bank: Option<()>, search_term: Option<&str>) {
        todo!()
    }

    fn get_resource(&self, id: ResourceId) -> Option<()> {
        None
    }

    fn preview(&self, action: &Action) {
        todo!()
    }

    fn commit(&mut self, action: Action) {
        todo!()
    }
}