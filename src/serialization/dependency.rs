use std::path::PathBuf;

use crate::utils::types::DependencyId;

/// Simple trait which describes an overtone dependency.
/// This trait will fuel power actions within the codebase, such as
/// renaming, moving, deleting, safely.
pub trait DependencyEntry {
    fn get_id(&self) -> DependencyId;
    fn get_path(&self) -> PathBuf;
}
