use crate::resource::{Resource, ResourceFieldError, ResourceFieldValue};

use self::serialization::ArrangementHeader;

pub mod dependency;
pub mod errors;
pub mod serialization;
pub mod time;

#[derive(Debug)]
pub struct Arrangement {
    pub header: ArrangementHeader,
}

impl<'a> Resource<'a> for Arrangement {
    fn get_field_ids() -> &'a [&'static str] {
        &["name"]
    }

    fn get_field_value(
        &self,
        field_id: &'static str,
    ) -> Result<ResourceFieldValue, ResourceFieldError> {
        match field_id {
            _ => Err(ResourceFieldError::FieldDoesntExist),
        }
    }

    fn set_field_value(
        &mut self,
        field_id: &'static str,
        value: ResourceFieldValue,
    ) -> Result<(), ResourceFieldError> {
        match field_id {
            "name" => match value {
                ResourceFieldValue::Text(t) => self.header.meta.name = t.to_string(),
                _ => return Err(ResourceFieldError::UnacceptableValue),
            },
            _ => return Err(ResourceFieldError::FieldDoesntExist),
        }

        Ok(())
    }

    fn save() -> Result<(), crate::resource::ResourceSaveError> {
        Ok(())
    }
}
