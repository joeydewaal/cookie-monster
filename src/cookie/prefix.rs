use std::borrow::Cow;

use super::{Cookie, CookieBuilder};

pub(crate) const HOST_PREFIX: &str = "__Host-";
pub(crate) const SECURE_PREFIX: &str = "__Secure-";

/// A recognized cookie name prefix as defined by
/// [RFC 6265bis §4.1.3](https://datatracker.ietf.org/doc/html/draft-ietf-httpbis-rfc6265bis#section-4.1.3).
///
/// This is an internal detail: the flavour is stored on a [`Cookie`], set only by
/// [`Cookie::host`] / [`Cookie::secure`] and by parsing, and re-applied on serialization.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CookiePrefix {
    Host,
    Secure,
}

impl CookiePrefix {
    /// The literal prefix string that is prepended to the name on the wire.
    pub(crate) const fn as_str(self) -> &'static str {
        match self {
            CookiePrefix::Host => HOST_PREFIX,
            CookiePrefix::Secure => SECURE_PREFIX,
        }
    }
}

/// Splits a recognized prefix off the front of a parsed cookie name. Matching is
/// case-sensitive per the spec, so `__host-` is not treated as a prefix. The returned name
/// is the logical (unprefixed) name.
pub(crate) fn split_prefix(name: Cow<'_, str>) -> (Option<CookiePrefix>, Cow<'_, str>) {
    let (prefix, len) = if name.starts_with(HOST_PREFIX) {
        (CookiePrefix::Host, HOST_PREFIX.len())
    } else if name.starts_with(SECURE_PREFIX) {
        (CookiePrefix::Secure, SECURE_PREFIX.len())
    } else {
        return (None, name);
    };

    let stripped = match name {
        Cow::Borrowed(borrowed) => Cow::Borrowed(&borrowed[len..]),
        Cow::Owned(mut owned) => {
            owned.drain(..len);
            Cow::Owned(owned)
        }
    };

    (Some(prefix), stripped)
}

impl Cookie {
    /// Builds a `__Host-` prefixed cookie.
    ///
    /// The `Secure` attribute is set and the `Path` attribute is set to `/`, and the `__Host-`
    /// prefix is applied to the name on serialization. These are the attributes the prefix
    /// requires per
    /// [RFC 6265bis §4.1.3](https://datatracker.ietf.org/doc/html/draft-ietf-httpbis-rfc6265bis#section-4.1.3),
    /// but they are only defaults: nothing stops you from changing the `Path`, adding a
    /// `Domain` or clearing `Secure` afterwards to build a non-standard cookie.
    ///
    /// The `name` you pass is the logical name; the prefix is stored separately and is not
    /// part of [`Cookie::name`].
    ///
    /// # Example
    /// ```rust
    /// use cookie_monster::Cookie;
    ///
    /// let cookie = Cookie::host("id", "abc").build();
    ///
    /// assert_eq!(cookie.name(), "id");
    /// assert!(cookie.is_secure());
    /// assert_eq!(cookie.path(), Some("/"));
    /// assert_eq!(cookie.serialize().as_deref(), Ok("__Host-id=abc; Path=/; Secure"));
    /// ```
    pub fn host<N, V>(name: N, value: V) -> CookieBuilder
    where
        N: Into<Cow<'static, str>>,
        V: Into<Cow<'static, str>>,
    {
        CookieBuilder::new(name, value)
            .secure()
            .path("/")
            .with_prefix(CookiePrefix::Host)
    }

    /// Builds a `__Secure-` prefixed cookie.
    ///
    /// The `Secure` attribute is set, and the `__Secure-` prefix is applied to the name on
    /// serialization. `Secure` is what the prefix requires per
    /// [RFC 6265bis §4.1.3](https://datatracker.ietf.org/doc/html/draft-ietf-httpbis-rfc6265bis#section-4.1.3),
    /// but it is only a default: you may clear it afterwards to build a non-standard cookie.
    ///
    /// The `name` you pass is the logical name; the prefix is stored separately and is not
    /// part of [`Cookie::name`].
    ///
    /// # Example
    /// ```rust
    /// use cookie_monster::Cookie;
    ///
    /// let cookie = Cookie::secure("id", "abc").build();
    ///
    /// assert_eq!(cookie.name(), "id");
    /// assert!(cookie.is_secure());
    /// assert_eq!(cookie.serialize().as_deref(), Ok("__Secure-id=abc; Secure"));
    /// ```
    pub fn secure<N, V>(name: N, value: V) -> CookieBuilder
    where
        N: Into<Cow<'static, str>>,
        V: Into<Cow<'static, str>>,
    {
        CookieBuilder::new(name, value)
            .secure()
            .with_prefix(CookiePrefix::Secure)
    }
}
