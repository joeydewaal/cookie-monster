use std::fmt::Display;

/// All errors that can be returned while parsing or serializing cookies.
#[derive(Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum Error {
    /// No '=' found in the cookie string.
    EqualsNotFound,
    /// Name value is empty.
    NameEmpty,
    /// Name contains invalid character.
    InvalidName(char),
    /// Value contains invalid character.
    InvalidValue(char),

    /// Unable to format the expires field.
    ExpiresFmt,

    /// Could not percent-decode the cookie.
    PercentDecodeError,

    /// Path attribute contains an invalid character.
    InvalidPathValue(char),
    /// Path attribute value is empty.
    EmptyPathValue,
    /// Path does not start with a leading '/'.
    NoLeadingSlash,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let err = match self {
            Error::EqualsNotFound => "No '=' found in the cookie",
            Error::NameEmpty => "The cookie name is empty",
            Error::InvalidName(c) => {
                return write!(f, "The cookie name contains an invalid character: {c}");
            }
            Error::InvalidValue(c) => {
                return write!(f, "The cookie value contains an invalid character: {c}");
            }
            Error::ExpiresFmt => "Failed to format the expires value",
            Error::PercentDecodeError => "An error occurred while decoding",
            Error::InvalidPathValue(c) => {
                return write!(f, "The path attribute contains an invalid character ({c})");
            }
            Error::EmptyPathValue => "The path attribute is empty",
            Error::NoLeadingSlash => "The path attribute does not start with a leading slash",
        };

        f.write_str(err)
    }
}

impl std::error::Error for Error {}
