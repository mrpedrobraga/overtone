use serde::{Deserialize, Serialize};
use ulid::Ulid;
use crate::Name;

/// Type that holds a unique identifier for a resource.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Ord, PartialOrd)]
#[derive(Serialize, Deserialize)]
pub struct ResourceId(Ulid);

/// Type that describes a new _kind_ of resource.
/// This is used whenever you have two resources of unknown type to describe if they are compatible.
///
/// It's recommented to namespace your formats with the name of your plugin.
/// So you may have `core:composition` `std:audio_clip`, `std:musx_data`, `thirdparty:pluginthing` etc.
#[repr(transparent)]
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
#[derive(Serialize, Deserialize)]
pub struct ResourceFormat(pub Name);

/// Trait for something that can search for and provide resources.
pub trait ResourceProvider {}

/// Header that represents an unloaded Resource.
#[derive(Clone, Eq, PartialEq, Debug)]
#[derive(Serialize, Deserialize)]
pub struct ResourceProviderHeader {
    pub name: Name,
    pub format: ResourceFormat,
    pub description: String,
}

/// Header that represents an unloaded Resource.
pub struct ResourceHeader {
    name: Name,
    description: String,
}