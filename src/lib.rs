mod cookie;
mod error;
mod jar;
mod util;

pub use cookie::{Cookie, CookieBuilder};
pub use error::Error;
pub(crate) type Result<T, E = Error> = ::std::result::Result<T, E>;
