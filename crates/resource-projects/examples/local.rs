use std::sync::Arc;
use std::vec::IntoIter;
use resource_projects::editor::{Editor, EditorClient};
use resource_projects::local::LocalEditor;
use resource_projects::project::Project;
use resource_projects::resource::{ResourceFormat, ResourceProviderHeader};

/// Example project that allows editing information about a family!
struct FamilyTreeProject {}

impl Project for FamilyTreeProject {
    fn list_resource_providers(&self) -> impl Iterator<Item=ResourceProviderHeader> {
        [
            ResourceProviderHeader {
                name: arcstr::literal!("famtree:person_provider"),
                format: ResourceFormat(arcstr::literal!("famtree:person")),
                description: String::from("A person, which can have relationships."),
            }
        ].into_iter()
    }
}

fn main() {
    let editor = LocalEditor::new(FamilyTreeProject {});
    let client = editor.new_client();
    let providers = client.list_resource_providers().collect::<Vec<_>>();

    dbg!(providers);
}