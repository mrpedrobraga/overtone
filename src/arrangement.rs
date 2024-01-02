use crate::{
    resource::{Resource, ResourceFieldError, ResourceFieldValue},
    serialization::arrangement::ArrangementHeader,
};
use std::fmt::Debug;

#[derive(Debug)]
pub struct Arrangement {
    header: ArrangementHeader,
}

impl<'a> Resource<'a> for Arrangement {
    fn get_field_ids() -> &'a [&'static str] {
        &["name"]
    }

    fn get_field_value(field_id: &'static str) -> Result<ResourceFieldValue, ResourceFieldError> {
        match field_id {
            _ => Err(ResourceFieldError::FieldDoesntExist),
        }
    }

    fn set_field_value(
        field_id: &'static str,
        value: ResourceFieldValue,
    ) -> Result<(), ResourceFieldError> {
        match field_id {
            _ => return Err(ResourceFieldError::FieldDoesntExist),
        }

        Ok(())
    }

    fn save() -> Result<(), crate::resource::ResourceSaveError> {
        Ok(())
    }
}
