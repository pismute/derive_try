use crate::error::Error;

use syn::{Data, DataStruct, DeriveInput, Field, Fields};

pub(crate) fn get_data_struct(input: &DeriveInput) -> Result<&DataStruct, Error> {
    match &input.data {
        Data::Struct(data) => Ok(data),
        Data::Enum(_) => Err(Error::NotForEnum),
        Data::Union(_) => Err(Error::NotForUnion),
    }
}

pub(crate) fn get_field(data: &DataStruct) -> Result<&Field, Error> {
    match &data.fields {
        Fields::Unit => Err(Error::FieldNotFound),
        Fields::Named(_) => Err(Error::UnsupportedNamedStruct),
        Fields::Unnamed(field) => {
            if field.unnamed.iter().count() > 1 {
                Err(Error::TooManyField)
            } else {
                field.unnamed.first().ok_or(Error::FieldNotFound)
            }
        }
    }
}
