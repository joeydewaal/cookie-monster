use std::{
    borrow::{Borrow, Cow},
    fmt,
    time::Duration,
};

use crate::Cookie;

use super::{expires::Expires, same_site::SameSite};

/// A builder struct for building a [`Cookie`].
#[derive(PartialEq, Clone)]
pub struct CookieBuilder(Cookie);

impl CookieBuilder {
    /// Build a new cookie. This returns a `CookieBuilder` that can be used to set other attribute
    /// values.
    ///
    /// # Example
    /// ```rust
    /// use cookie_monster::CookieBuilder;
    ///
    /// let cookie = CookieBuilder::new("foo", "bar")
    ///     .secure()
    ///     .http_only()
    ///     .build();
    ///
    /// assert!(cookie.secure());
    /// assert!(cookie.http_only());
    /// ```
    #[inline]
    pub fn new<N, V>(name: N, value: V) -> CookieBuilder
    where
        N: Into<Cow<'static, str>>,
        V: Into<Cow<'static, str>>,
    {
        CookieBuilder(Cookie::new(name, value))
    }

    /// Sets the name of the cookie.
    #[inline]
    pub fn name<N: Into<Cow<'static, str>>>(mut self, name: N) -> Self {
        self.0.set_name(name);
        self
    }

    /// Returns the name of the cookie.
    pub fn get_name(&self) -> &str {
        self.0.name()
    }

    /// Sets the value of the cookie.
    #[inline]
    pub fn value<V: Into<Cow<'static, str>>>(mut self, value: V) -> Self {
        self.0.set_value(value);
        self
    }

    /// Returns the value of the cookie.
    pub fn get_value(&self) -> &str {
        self.0.value()
    }

    /// Sets the Expires attribute of the cookie.
    ///
    /// The argument can be a few different types, based on what features are enabled.
    ///
    /// # No features needed
    ///
    /// ```rust
    /// use cookie_monster::{Cookie, Expires};
    ///
    /// let cookie = Cookie::build("foo", "bar")
    ///     .expires(Expires::remove())
    ///     .build();
    ///
    /// assert!(cookie.is_expires_set());
    /// ```
    ///
    /// # Jiff
    /// ```rust
    /// # #[cfg(feature="jiff")]
    /// # {
    /// # use cookie_monster::Cookie;
    /// use jiff::Zoned;
    ///
    /// let cookie = Cookie::build("foo", "bar")
    ///     .expires(Zoned::now())
    ///     .build();
    ///
    /// # assert!(cookie.is_expires_set());
    /// # }
    /// ```
    ///
    /// # Chrono
    /// ```rust
    /// # #[cfg(feature="chrono")]
    /// # {
    /// # use cookie_monster::Cookie;
    /// use chrono::Utc;
    ///
    /// let cookie = Cookie::build("foo", "bar")
    ///     .expires(Utc::now())
    ///     .build();
    ///
    /// # assert!(cookie.is_expires_set());
    /// # }
    /// ```
    ///
    /// # Time
    /// ```rust
    /// # #[cfg(feature="time")]
    /// # {
    /// # use cookie_monster::Cookie;
    /// use time::OffsetDateTime;
    ///
    /// let cookie = Cookie::build("foo", "bar")
    ///     .expires(OffsetDateTime::now_utc())
    ///     .build();
    ///
    /// # assert!(cookie.is_expires_set());
    /// # }
    /// ```
    #[inline]
    pub fn expires(mut self, expiration: impl Into<Expires>) -> Self {
        self.0.set_expires(expiration.into());
        self
    }

    /// Sets the Max-Age attribute of the cookie.
    ///
    /// # Example
    /// ```rust
    /// use cookie_monster::Cookie;
    ///
    /// let cookie = Cookie::build("foo", "bar")
    ///     .max_age_secs(100)
    ///     .build();
    ///
    /// assert_eq!(cookie.max_age_secs(), Some(100));
    /// ```
    #[inline]
    pub fn max_age_secs(mut self, max_age_secs: u64) -> Self {
        self.0.set_max_age_secs(max_age_secs);
        self
    }

    /// Sets the Max-Age attribute of the cookie.
    ///
    /// # Example
    /// ```rust
    /// use cookie_monster::Cookie;
    /// use std::time::Duration;
    ///
    /// let cookie = Cookie::build("foo", "bar")
    ///     .max_age(Duration::from_secs(100))
    ///     .build();
    ///
    /// assert_eq!(cookie.max_age(), Some(Duration::from_secs(100)));
    /// ```
    #[inline]
    pub fn max_age(mut self, max_age: Duration) -> Self {
        self.0.set_max_age(max_age);
        self
    }

    /// Sets the Domain attribute of the cookie.
    ///
    /// # Note
    /// If the domain attribute is set to an empty string or the string contains an invalid cookie
    /// character, the attribute is ignored.
    ///
    /// # Example
    /// ```rust
    /// use cookie_monster::Cookie;
    ///
    /// let cookie = Cookie::build("foo", "bar")
    ///     .domain("rust-lang.com")
    ///     .build();
    ///
    /// assert_eq!(cookie.domain(), Some("rust-lang.com"));
    /// ```
    #[inline]
    pub fn domain<D: Into<Cow<'static, str>>>(mut self, domain: D) -> Self {
        self.0.set_domain(domain);
        self
    }

    /// Sets the Path attribute of the cookie.
    ///
    /// # Note
    /// Not all path value's are allowed by the standard:
    /// * The path can't be set to and empty string.
    /// * The path must start with a leading `/`.
    /// * The path can't contain invalid cookie characters.
    ///
    /// If any of these conditions are not met, serializing this cookie returns an error.
    ///
    /// # Example
    /// ```rust
    /// use cookie_monster::Cookie;
    ///
    /// let cookie = Cookie::build("foo", "bar")
    ///     .path("/api/login")
    ///     .build();
    ///
    /// assert_eq!(cookie.path(), Some("/api/login"));
    /// ```
    #[inline]
    pub fn path<D: Into<Cow<'static, str>>>(mut self, path: D) -> Self {
        self.0.set_path(path);
        self
    }

    /// Sets the Secure attribute of the cookie.
    ///
    /// # Example
    /// ```rust
    /// use cookie_monster::Cookie;
    ///
    /// let cookie = Cookie::build("foo", "bar")
    ///     .secure()
    ///     .build();
    ///
    /// assert!(cookie.secure());
    /// ```
    #[inline]
    pub fn secure(mut self) -> Self {
        self.0.set_secure(true);
        self
    }

    /// Sets the Secure attribute.
    #[inline]
    pub fn set_secure(mut self, secure: bool) -> Self {
        self.0.set_secure(secure);
        self
    }

    /// Sets the HttpOnly attribute of the cookie.
    ///
    /// # Example
    /// ```rust
    /// use cookie_monster::Cookie;
    ///
    /// let cookie = Cookie::build("foo", "bar")
    ///     .http_only()
    ///     .build();
    ///
    /// assert!(cookie.http_only());
    /// ```
    #[inline]
    pub fn http_only(mut self) -> Self {
        self.0.set_http_only(true);
        self
    }

    /// Sets the HttpOnly attribute of the cookie.
    #[inline]
    pub fn set_http_only(mut self, http_only: bool) -> Self {
        self.0.set_http_only(http_only);
        self
    }

    /// Sets the Partitioned attribute of the cookie. When the partitioned attribute is enabled, the
    /// secure flag is also enabled while serializing.
    ///
    /// <https://developer.mozilla.org/en-US/docs/Web/Privacy/Guides/Privacy_sandbox/Partitioned_cookies>
    ///
    /// # Example
    /// ```rust
    /// use cookie_monster::Cookie;
    ///
    /// let cookie = Cookie::build("foo", "bar")
    ///     .partitioned()
    ///     .build();
    ///
    /// assert!(cookie.partitioned());
    /// ```
    #[inline]
    pub fn partitioned(self) -> Self {
        self.set_partitioned(true)
    }

    /// Set the Partitioned flag, enabling the Partitioned attribute also enables the Secure Attribute.
    #[inline]
    pub fn set_partitioned(mut self, partitioned: bool) -> Self {
        self.0.set_partitioned(partitioned);
        self
    }

    /// Sets the SameSite attribute value of the cookie.
    ///
    /// # Example
    /// ```rust
    /// use cookie_monster::{Cookie, SameSite};
    ///
    /// let cookie = Cookie::build("foo", "bar")
    ///     .same_site(SameSite::Strict)
    ///     .build();
    ///
    /// assert_eq!(cookie.same_site(), Some(SameSite::Strict));
    /// ```
    #[inline]
    pub fn same_site<S: Into<Option<SameSite>>>(mut self, same_site: S) -> Self {
        self.0.set_same_site(same_site);
        self
    }

    /// Builds and returns the cookie
    #[inline]
    pub fn build(self) -> Cookie {
        self.0
    }
}

impl fmt::Debug for CookieBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

impl fmt::Display for CookieBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

impl Borrow<Cookie> for CookieBuilder {
    fn borrow(&self) -> &Cookie {
        &self.0
    }
}

impl Into<Cookie> for CookieBuilder {
    fn into(self) -> Cookie {
        self.0
    }
}
