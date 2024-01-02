use crate::types::RefStr;

/// Trait that allows a value to be edited from a generic inspector.
pub trait Resource<'a> {
    fn get_field_ids() -> &'a [&'static str];

    fn get_field_value(field_id: &'static str) -> Result<ResourceFieldValue, ResourceFieldError>;

    fn set_field_value(
        field_id: &'static str,
        value: ResourceFieldValue,
    ) -> Result<(), ResourceFieldError>;

    fn save() -> Result<(), ResourceSaveError>;
}

#[derive(Debug)]
pub enum ResourceFieldValue {
    Text(RefStr),
}

pub enum ResourceFieldError {
    FieldDoesntExist,
    UnacceptableValue,
}

pub enum ResourceSaveError {
    GenericError,
}
