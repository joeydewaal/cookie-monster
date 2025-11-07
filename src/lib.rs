//! An HTTP Cookie crate.
//!
//! # Overview
//! Exposes types like [`Cookie`] and [`CookieJar`] for working with HTTP cookies. This crate
//! focuses on server side applications. The main goals are simplicity and ease of use.
//!
//! # Usage
//! Add cookie-monster in your Cargo.toml:
//! ```toml
//![dependencies]
//!cookie-monster = "0.1"
//!```
//!
//! # Features
//! * `jiff`
//!
//!   Adds support for the [jiff](https://docs.rs/jiff/latest/jiff/) crate.
//!   This exposes methods to [`Cookie`] to retrieve the `Expires` and `Max-Age` attributes with jiff
//!   specific types.
//!
//! * `chrono`
//!
//!   Adds support for the [chrono](https://docs.rs/chrono/latest/chrono/) crate.
//!   This exposes methods to [`Cookie`] to retrieve the `Expires` and `Max-Age` attributes with
//!   chrono specific types.
//!
//! * `time`
//!
//!   Adds support for the [time](https://docs.rs/time/latest/time/index.html) crate.
//!   This exposes methods to [`Cookie`] to retrieve the `Expires` and `Max-Age` attributes with time
//!   specific types.
//!
//! * `percent-encode`
//!
//!   Parse/serialize [`Cookie`]s that are percent-encoded.
//!
//! * `axum`
//!
//!   Adds integration with the [axum](https://docs.rs/axum/latest/axum/) crate.
//!   Implements [`FromRequestParts`](https://docs.rs/axum/latest/axum/extract/trait.FromRequestParts.html),
//!   [`IntoResponse`](https://docs.rs/axum/latest/axum/response/trait.IntoResponse.html) and
//!   [`IntoResponseParts`](https://docs.rs/axum/latest/axum/response/trait.IntoResponseParts.html),

//!
//! * `http`
//!
//!   Adds integration with the [http](https://docs.rs/http/latest/http/) crate.
//!   Create a [`CookieJar`] from a [`HeaderMap`](https://docs.rs/http/latest/http/header/struct.HeaderMap.html).
//!   Write a [`CookieJar`] to a [`HeaderMap`](https://docs.rs/http/latest/http/header/struct.HeaderMap.html).
//!
//!
//! # Axum example
//!
//! ```rust
//! use axum::response::IntoResponse;
//! use cookie_monster::{Cookie, CookieJar, SameSite};
//!
//! static COOKIE_NAME: &str = "session";
//!
//! async fn handler(mut jar: CookieJar) -> impl IntoResponse {
//!     if let Some(cookie) = jar.get(COOKIE_NAME) {
//!         // Remove cookie
//!         println!("Removing cookie {cookie:?}");
//!         jar.remove(Cookie::named(COOKIE_NAME));
//!     } else {
//!         // Set cookie.
//!         let cookie = Cookie::build(COOKIE_NAME, "hello, world")
//!         .http_only()
//!         .same_site(SameSite::Strict);
//!
//!         println!("Setting cookie {cookie:?}");
//!         jar.add(cookie);
//!     }
//!     // Return the jar so the cookies are updated
//!    jar
//! }
//! ```
//!
//!
//! ### Honorable mention
//! This crate takes a lot of inspiration from the [cookie](https://crates.io/crates/cookie) crate.

mod cookie;
mod error;
mod jar;
mod util;

#[cfg(feature = "axum")]
mod axum;

#[cfg(feature = "http")]
mod http;

pub use cookie::{Cookie, CookieBuilder, expires::Expires, same_site::SameSite};
pub use error::Error;
pub(crate) type Result<T, E = Error> = ::std::result::Result<T, E>;
pub use jar::CookieJar;
