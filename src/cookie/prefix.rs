use std::borrow::Cow;
use std::fmt::Display;

use super::{Cookie, CookieBuilder};

pub(crate) const HOST_PREFIX: &str = "__Host-";
pub(crate) const SECURE_PREFIX: &str = "__Secure-";

/// A recognized cookie name prefix as defined by
/// [RFC 6265bis §4.1.3](https://datatracker.ietf.org/doc/html/draft-ietf-httpbis-rfc6265bis#section-4.1.3).
///
/// The prefix is stored on the [`Cookie`] and can be read with [`Cookie::prefix`]. It is
/// detected from the cookie name, both when a cookie is created and when one is parsed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CookiePrefix {
    /// The `__Host-` prefix.
    Host,
    /// The `__Secure-` prefix.
    Secure,
}

/// Detects the prefix of a cookie name. Matching is case-sensitive per the spec, so
/// `__host-` is not treated as a prefix.
pub(crate) fn detect(name: &str) -> Option<CookiePrefix> {
    if name.starts_with(HOST_PREFIX) {
        Some(CookiePrefix::Host)
    } else if name.starts_with(SECURE_PREFIX) {
        Some(CookiePrefix::Secure)
    } else {
        None
    }
}

impl Cookie {
    /// Builds a `__Host-` prefixed cookie.
    ///
    /// The `__Host-` prefix is prepended to `name`, the `Secure` attribute is set and the
    /// `Path` attribute is set to `/`. These are the attributes the prefix requires per
    /// [RFC 6265bis §4.1.3](https://datatracker.ietf.org/doc/html/draft-ietf-httpbis-rfc6265bis#section-4.1.3),
    /// but they are only defaults: nothing stops you from changing the `Path`, adding a
    /// `Domain` or clearing `Secure` afterwards to build a non-standard cookie.
    ///
    /// The [`CookiePrefix::Host`] flavour is stored on the cookie and can be read with
    /// [`Cookie::prefix`].
    ///
    /// # Example
    /// ```rust
    /// use cookie_monster::Cookie;
    ///
    /// let cookie = Cookie::host("id", "abc").build();
    ///
    /// assert_eq!(cookie.name(), "__Host-id");
    /// assert!(cookie.is_secure());
    /// assert_eq!(cookie.path(), Some("/"));
    /// assert_eq!(cookie.serialize().as_deref(), Ok("__Host-id=abc; Path=/; Secure"));
    /// ```
    pub fn host<V>(name: impl Display, value: V) -> CookieBuilder
    where
        V: Into<Cow<'static, str>>,
    {
        let name = format!("{HOST_PREFIX}{name}");
        CookieBuilder::new(name, value).secure().path("/")
    }

    /// Builds a `__Secure-` prefixed cookie.
    ///
    /// The `__Secure-` prefix is prepended to `name` and the `Secure` attribute is set. The
    /// `Secure` attribute is what the prefix requires per
    /// [RFC 6265bis §4.1.3](https://datatracker.ietf.org/doc/html/draft-ietf-httpbis-rfc6265bis#section-4.1.3),
    /// but it is only a default: you may clear it afterwards to build a non-standard cookie.
    ///
    /// The [`CookiePrefix::Secure`] flavour is stored on the cookie and can be read with
    /// [`Cookie::prefix`].
    ///
    /// # Example
    /// ```rust
    /// use cookie_monster::Cookie;
    ///
    /// let cookie = Cookie::secure("id", "abc").build();
    ///
    /// assert_eq!(cookie.name(), "__Secure-id");
    /// assert!(cookie.is_secure());
    /// assert_eq!(cookie.serialize().as_deref(), Ok("__Secure-id=abc; Secure"));
    /// ```
    pub fn secure<V>(name: impl Display, value: V) -> CookieBuilder
    where
        V: Into<Cow<'static, str>>,
    {
        let name = format!("{SECURE_PREFIX}{name}");
        CookieBuilder::new(name, value).secure()
    }
}
