use std::{
    borrow::Cow,
    fmt::{self, Debug},
    time::Duration,
};

mod builder;
mod domain;
pub(crate) mod expires;
mod parse;
mod path;
pub(crate) mod same_site;
mod serialize;

#[cfg(feature = "percent-encode")]
mod encoding;

pub use builder::CookieBuilder;
use expires::Expires;

use crate::{SameSite, util::TinyStr};

/// An HTTP Cookie.
#[derive(Default, Clone)]
pub struct Cookie
{
    // A read only buffer to the raw cookie value.
    raw_value: Option<String>,
    name: TinyStr,
    value: TinyStr,
    expires: Expires,
    max_age: Option<u64>,
    domain: Option<TinyStr>,
    path: Option<TinyStr>,
    secure: bool,
    http_only: bool,
    partitioned: bool,
    same_site: Option<SameSite>,
}

impl Cookie {
    /// Creates a new cookie with the given name and value.
    ///
    /// # Example
    /// ```rust
    /// use cookie_monster::Cookie;
    ///
    /// let cookie = Cookie::new("hello", "world");
    ///
    /// assert_eq!(cookie.name(), "hello");
    /// assert_eq!(cookie.value(), "world");
    /// ```
    pub fn new<N, V>(name: N, value: V) -> Cookie
    where
        N: Into<Cow<'static, str>>,
        V: Into<Cow<'static, str>>,
    {
        Self::new_inner(TinyStr::from(name), TinyStr::from(value))
    }

    /// Creates a cookie that can be used to remove the cookie from the user-agent. This sets the
    /// Expires attribute in the past and Max-Age to 0 seconds.
    ///
    /// **To ensure a cookie is removed from the user-agent, set the `Path` and `Domain` attributes
    /// with the same values that were used to create the cookie.**
    ///
    /// # Note
    /// You don't have to use this method with [`crate::CookieJar::remove`], the jar automatically set's
    /// the Expires and Max-Age attributes.
    ///
    /// # Example
    /// ```rust
    /// use cookie_monster::Cookie;
    ///
    /// let cookie = Cookie::remove("session");
    ///
    /// assert_eq!(cookie.max_age_secs(), Some(0));
    /// assert!(!cookie.is_expires_set());
    /// ```
    pub fn remove<N>(name: N) -> Cookie
    where
        N: Into<Cow<'static, str>>,
    {
        Cookie::new(name, "").into_remove()
    }

    pub(crate) fn into_remove(mut self) -> Self {
        self.set_expires(Expires::remove());
        self.set_max_age_secs(0);
        self.set_value("");
        self
    }

    fn new_inner(name: TinyStr, value: TinyStr) -> Cookie {
        Cookie {
            name,
            value,
            ..Default::default()
        }
    }

    /// Build a new cookie. This returns a `CookieBuilder` that can be used to set other attribute
    /// values.
    ///
    /// # Example
    /// ```rust
    /// use cookie_monster::Cookie;
    ///
    /// let cookie = Cookie::build("foo", "bar")
    ///     .secure()
    ///     .http_only()
    ///     .build();
    ///
    /// assert!(cookie.secure());
    /// assert!(cookie.http_only());
    /// ```
    pub fn build<N, V>(name: N, value: V) -> CookieBuilder
    where
        N: Into<Cow<'static, str>>,
        V: Into<Cow<'static, str>>,
    {
        CookieBuilder::new(name, value)
    }

    /// Creates a [`CookieBuilder`] with the given name and an empty value. This can be used when
    /// removing a cookie from a [`crate::CookieJar`].
    ///
    /// # Example
    /// ```rust
    /// use cookie_monster::{Cookie, CookieJar};
    ///
    /// let mut jar = CookieJar::empty();
    /// jar.remove(Cookie::named("session").path("/login"));
    ///
    /// assert!(jar.get("session").is_none());
    /// ```
    pub fn named<N>(name: N) -> CookieBuilder
    where
        N: Into<Cow<'static, str>>,
    {
        Self::build(name, "")
    }

    /// Returns the cookie name.
    #[inline]
    pub fn name(&self) -> &str {
        self.name.as_str(self.raw_value.as_deref())
    }

    /// Set the cookie name.
    #[inline]
    pub fn set_name<N: Into<Cow<'static, str>>>(&mut self, name: N) {
        self.name = TinyStr::from(name)
    }

    /// Get the cookie value. This does not trim `"` characters.
    #[inline]
    pub fn value(&self) -> &str {
        self.value.as_str(self.raw_value.as_deref())
    }

    /// Set the cookie value.
    #[inline]
    pub fn set_value<V: Into<Cow<'static, str>>>(&mut self, value: V) {
        self.value = TinyStr::from(value)
    }

    /// Set the Expired attribute.
    #[inline]
    pub fn set_expires<E: Into<Expires>>(&mut self, expires: E) {
        self.expires = expires.into();
    }

    /// Get the Max-Age duration. This returns a `std::time::Duration`, if you'd like a `time`,
    /// `chrono` or `jiff` specific duration use the `max_age_{time,chrono,jiff}` method.
    #[inline]
    pub fn max_age(&self) -> Option<Duration> {
        self.max_age.map(Duration::from_secs)
    }

    /// Get the Max-Age as seconds.
    #[inline]
    pub fn max_age_secs(&self) -> Option<u64> {
        self.max_age
    }

    /// Set the Max-Age attribute.
    #[inline]
    pub fn set_max_age(&mut self, max_age: Duration) {
        self.set_max_age_secs(max_age.as_secs());
    }

    /// Set the Max-Age value in seconds.
    #[inline]
    pub fn set_max_age_secs(&mut self, max_age_secs: u64) {
        self.max_age = Some(max_age_secs);
    }

    /// Removes the Max-Age attribute.
    #[inline]
    pub fn unset_max_age(&mut self) {
        self.max_age = None;
    }

    /// Returns the Domain attribute if it's set.
    #[inline]
    pub fn domain(&self) -> Option<&str> {
        self.domain
            .as_ref()
            .map(|s| s.as_str(self.raw_value.as_deref()))
    }

    pub(crate) fn domain_sanitized(&self) -> Option<&str> {
        self.domain().map(|d| d.strip_prefix('.').unwrap_or(d))
    }

    /// Set the Domain attribute.
    #[inline]
    pub fn set_domain<D: Into<Cow<'static, str>>>(&mut self, domain: D) {
        self.domain = Some(TinyStr::from(domain))
    }

    /// Removes the Domain attribute.
    #[inline]
    pub fn unset_domain(&mut self) {
        self.domain = None
    }

    /// Returns the Path attribute if it's set.
    #[inline]
    pub fn path(&self) -> Option<&str> {
        self.path
            .as_ref()
            .map(|val| val.as_str(self.raw_value.as_deref()))
    }

    /// Set the Path attribute.
    #[inline]
    pub fn set_path<D: Into<Cow<'static, str>>>(&mut self, path: D) {
        self.path = Some(TinyStr::from(path))
    }

    /// Removes the path attribute.
    #[inline]
    pub fn unset_path(&mut self) {
        self.path = None
    }

    /// Returns if the Secure attribute is set.
    #[inline]
    pub fn secure(&self) -> bool {
        self.secure
    }

    /// Sets the Secure attribute of the cookie.
    #[inline]
    pub fn set_secure(&mut self, secure: bool) {
        self.secure = secure
    }

    /// Returns if the HttpOnly attribute is set.
    #[inline]
    pub fn http_only(&self) -> bool {
        self.http_only
    }

    /// Sets the HttpOnly attribute of the cookie.
    #[inline]
    pub fn set_http_only(&mut self, http_only: bool) {
        self.http_only = http_only
    }

    /// Returns if the Partitioned attribute is set.
    #[inline]
    pub fn partitioned(&self) -> bool {
        self.partitioned
    }

    /// Set the Partitioned flag, enabling the Partitioned attribute also enables the Secure Attribute.
    #[inline]
    pub fn set_partitioned(&mut self, partitioned: bool) {
        self.partitioned = partitioned;
    }

    /// Returns the SameSite attribute if it is set.
    #[inline]
    pub fn same_site(&self) -> Option<SameSite> {
        self.same_site
    }

    /// Set the SameSite attribute.
    #[inline]
    pub fn set_same_site<S: Into<Option<SameSite>>>(&mut self, same_site: S) {
        self.same_site = same_site.into();
    }
}

impl fmt::Display for Cookie {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Debug for Cookie {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug = f.debug_struct("Cookie");

        debug
            .field("name", &self.name())
            .field("value", &self.value())
            .field("max_age", &self.max_age())
            .field("domain", &self.domain())
            .field("path", &self.path())
            .field("secure", &self.secure())
            .field("http_only", &self.http_only())
            .field("partitioned", &self.partitioned())
            .field("expires", &self.expires)
            .finish()
    }
}

impl PartialEq<Cookie> for Cookie {
    fn eq(&self, other: &Cookie) -> bool {
        if self.name() != other.name()
            || self.value() != other.value()
            || self.secure() != other.secure()
            || self.http_only() != other.http_only()
            || self.partitioned() != other.partitioned()
            || self.max_age() != other.max_age()
            || self.same_site() != other.same_site()
            || self.expires != other.expires
        {
            return false;
        }

        if !opt_str_eq(self.domain_sanitized(), other.domain_sanitized()) {
            return false;
        }

        if !opt_str_eq(self.path(), other.path()) {
            return false;
        }

        true
    }
}

fn opt_str_eq(left: Option<&str>, right: Option<&str>) -> bool {
    match (left, right) {
        (None, None) => true,
        (Some(l), Some(r)) => l.eq_ignore_ascii_case(r),
        _ => false,
    }
}
