use std::num::ParseIntError;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    // Name - value
    EqualsNotFound, // No '=' found in name-value pair
    NameEmpty,      // Cookie name is empty
    InvalidName,    // Cookie name contains invalid character
    InvalidValue,   // Cookie value contains invalid character

    // Attributes
    UnkownAttribute,

    // Expires
    ExpiresFmt,

    // cookie-value
    InvalidAttribute,

    // Max-Age
    MaxAgeValueMissing, // No Max-Age value found
    MaxAgeValueParse,

    // Domain
    DomainEmpty,

    // Path
    PathEmpty,
}

impl From<ParseIntError> for Error {
    fn from(_: ParseIntError) -> Self {
        Error::MaxAgeValueParse
    }
}
