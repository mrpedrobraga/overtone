use crate::{serialization::dependency::DependencyEntry, utils::types::DependencyId};
use serde_derive::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
pub struct ArrFragmentReference {
    pub id: DependencyId,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ArrFragmentDependencyEntry {
    pub id: DependencyId,
    pub path: PathBuf,
}

impl DependencyEntry for ArrFragmentDependencyEntry {
    fn get_id(&self) -> DependencyId {
        self.id.clone()
    }

    fn get_path(&self) -> PathBuf {
        self.path.clone()
    }
}
