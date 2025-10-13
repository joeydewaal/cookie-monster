mod cookie;
mod error;
mod jar;
mod util;

#[cfg(feature = "axum")]
mod axum;

#[cfg(feature = "http")]
mod http;

pub use cookie::{Cookie, CookieBuilder, same_site::SameSite};
pub use error::Error;
pub(crate) type Result<T, E = Error> = ::std::result::Result<T, E>;
pub use jar::CookieJar;
