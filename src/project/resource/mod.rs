//! # Resource Management
//!
//! A [`Resource`] is something that the program uses / can edit.
//!
//! Because different kinds of resources can be loaded at runtime,
//! the `Resource` trait offers a light reflection API.

use crate::RefStr;

/// Trait that allows a value to be edited from a generic inspector.
pub trait Resource<'a> {
    /// Returns all the fields available for editing.
    fn get_fields_info() -> &'a [ResourceFieldInfo];

    /// Returns the value of a specific field given its ID.
    ///
    /// This can fail if the field doesn't exist.
    fn get_field_value(
        &self,
        field_id: &'static str,
    ) -> Result<ResourceFieldValue, ResourceGetFieldError>;

    /// Sets the value of a field given its ID.
    ///
    /// This can fail if the field doesn't exist or
    /// refuses to accept a value of the given type.
    fn set_field_value(
        &mut self,
        field_id: &'static str,
        value: ResourceFieldValue,
    ) -> Result<(), ResourceSetFieldError>;

    fn save() -> Result<(), ResourceSaveError>;
}

pub struct ResourceFieldInfo {
    pub name: &'static str,
}

#[derive(Debug)]
#[non_exhaustive]
/// A value that can be stored or retrieved from a Resource.
pub enum ResourceFieldValue {
    Text(RefStr),
    F32(f32),
    Bool(bool),
}

/// Error originated from attempting to set the value of a field in a resource.
pub enum ResourceSetFieldError {
    /// Field doesn't exist.
    NoSuchField
}

/// Error originated from attempting to retrieve the value of a field in a resource.
pub enum ResourceGetFieldError {
    /// Field doesn't exist.
    NoSuchField,
    /// Field doesn't accept a value of the given type.
    IncompatibleType,
}

/// Error originated from trying to save a resource to disk.
pub enum ResourceSaveError {
    GenericError,
}
