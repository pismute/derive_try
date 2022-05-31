use std::string::ToString;

pub(crate) enum Error {
    NotForEnum,
    NotForUnion,
    FieldNotFound,
    TooManyField,
    UnsupportedNamedStruct,
}

impl ToString for Error {
    fn to_string(&self) -> String {
        match self {
            Error::NotForEnum => "Enums are not supported".to_string(),
            Error::NotForUnion => "Unions are not supported".to_string(),
            Error::FieldNotFound => "At least one field is required".to_string(),
            Error::TooManyField => "too many fields, must be single field".to_string(),
            Error::UnsupportedNamedStruct => "Named structs are not supported".to_string(),
        }
    }
}
