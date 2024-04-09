pub(crate) enum Error {
    NotForEnum,
    NotForUnion,
    FieldNotFound,
    TooManyField,
    UnsupportedNamedStruct,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::NotForEnum => write!(f, "Enums are not supported"),
            Error::NotForUnion => write!(f, "Unions are not supported"),
            Error::FieldNotFound => write!(f, "At least one field is required"),
            Error::TooManyField => write!(f, "too many fields, must be single field"),
            Error::UnsupportedNamedStruct => write!(f, "Named structs are not supported"),
        }
    }
}
