use std::fmt::Display;

/// All errors that can be returned while parsing or serializing cookies.
#[derive(Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum Error {
    // Name - value
    EqualsNotFound,
    NameEmpty,
    InvalidName,
    InvalidValue,

    // Expires
    ExpiresFmt,

    // cookie-value
    PercentDecodeError,

    // Path
    InvalidPathValue,
    EmptyPathValue,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let err = match self {
            Error::EqualsNotFound => "No '=' found in the cookie",
            Error::NameEmpty => "The cookie name is empty",
            Error::InvalidName => "The cookie name contains an invalid character",
            Error::InvalidValue => "The cookie value contains an invalid value",
            Error::ExpiresFmt => "Failed to format the expires value",
            Error::PercentDecodeError => "An error occured while decoding",
            Error::InvalidPathValue => "The path attribute contains an invalid value",
            Error::EmptyPathValue => "The path attribute is empty",
        };

        f.write_str(err)
    }
}

impl std::error::Error for Error {}
