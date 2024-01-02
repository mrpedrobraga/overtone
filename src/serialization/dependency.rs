use serde_derive::{Deserialize, Serialize};

/// Simple trait which describes an overtone dependency.
/// This trait will fuel power actions within the codebase, such as
/// renaming, moving, deleting, safely.
pub trait DependencyEntry {
    fn get_id(&self) -> String;
    fn get_path(&self) -> String;
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PluginDependencyEntry {
    pub id: String,
    pub path: String,
}

impl DependencyEntry for PluginDependencyEntry {
    fn get_id(&self) -> String {
        self.id.clone()
    }

    fn get_path(&self) -> String {
        self.path.clone()
    }
}
