//! # Project
//!
//! It holds several kinds of resources within itself.
//!
//! This trait will always be implemented by the user of the library,
//! as that implementation is what

use crate::resource::ResourceProviderHeader;

/// The main trait of this module.
pub trait Project {
    /// Returns the kind of resources this project supports.
    fn list_resource_providers(&self) -> impl Iterator<Item = ResourceProviderHeader>;
}