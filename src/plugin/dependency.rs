use crate::utils::DependencyId;
use serde_derive::{Deserialize, Serialize};
use std::path::PathBuf;
use crate::project::serialization::dependency::DependencyEntry;

#[derive(Serialize, Deserialize, Debug)]
pub struct PluginDependencyEntry {
    pub id: DependencyId,
    pub path: PathBuf,
}

impl DependencyEntry for PluginDependencyEntry {
    fn get_id(&self) -> DependencyId {
        self.id.clone()
    }

    fn get_path(&self) -> PathBuf {
        self.path.clone()
    }
}
