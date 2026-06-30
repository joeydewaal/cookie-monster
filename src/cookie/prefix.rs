use std::borrow::Cow;
use std::fmt::Display;

use crate::Error;

use super::{Cookie, CookieBuilder};

const HOST_PREFIX: &str = "__Host-";
const SECURE_PREFIX: &str = "__Secure-";

/// A recognized cookie name prefix as defined by RFC 6265bis §4.1.3.
pub(crate) enum CookiePrefix {
    Host,
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
    /// `Path` attribute is set to `/`. A `__Host-` cookie must not have a `Domain` attribute.
    ///
    /// If the returned cookie is later mutated so it violates these rules (for example by
    /// adding a `Domain`, changing the `Path`, or clearing `Secure`), serializing it fails
    /// with a detailed [`Error`](crate::Error).
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
    /// The `__Secure-` prefix is prepended to `name` and the `Secure` attribute is set.
    ///
    /// If the returned cookie is later mutated so it violates these rules (for example by
    /// clearing `Secure`), serializing it fails with a detailed [`Error`](crate::Error).
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

    /// Validates the cookie against the rules implied by its name prefix.
    ///
    /// Returns `Ok(())` when the cookie has no recognized prefix.
    pub(crate) fn check_prefix(&self) -> crate::Result<()> {
        match detect(self.name()) {
            Some(CookiePrefix::Host) => {
                if !self.is_secure() {
                    return Err(Error::HostPrefixNotSecure);
                }
                if self.domain().is_some() {
                    return Err(Error::HostPrefixHasDomain);
                }
                if self.path() != Some("/") {
                    return Err(Error::HostPrefixBadPath);
                }
                Ok(())
            }
            Some(CookiePrefix::Secure) => {
                if !self.is_secure() {
                    return Err(Error::SecurePrefixNotSecure);
                }
                Ok(())
            }
            None => Ok(()),
        }
    }
}
