#[derive(Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum Error {
    // Name - value
    EqualsNotFound, // No '=' found in name-value pair
    NameEmpty,      // Cookie name is empty
    InvalidName,    // Cookie name contains invalid character
    InvalidValue,   // Cookie value contains invalid character

    // Expires
    ExpiresFmt,

    // cookie-value
    InvalidAttribute,

    // Path
    InvalidPathValue,
    EmptyPathValue,
}
