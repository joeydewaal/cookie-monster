use std::{
    borrow::Cow,
    fmt::{self, Debug},
    time::Duration,
};

mod builder;
mod domain;
mod expires;
mod max_age;
mod parse;
mod path;
pub(crate) mod same_site;
mod serialize;

#[cfg(feature = "percent-encode")]
mod encoding;

pub use builder::CookieBuilder;
use expires::Expires;

use crate::{SameSite, util::TinyStr};

#[derive(Default, Clone)]
pub struct Cookie {
    // A read only buffer to the raw cookie value.
    raw_value: Option<Box<str>>,
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
    // Creates a new cookie with the given name and value.
    pub fn new<N, V>(name: N, value: V) -> Cookie
    where
        N: Into<Cow<'static, str>>,
        V: Into<Cow<'static, str>>,
    {
        Self::new_inner(TinyStr::from(name), TinyStr::from(value))
    }

    // Creates a cookie that removes the cookie with the given name.
    pub fn remove<N>(name: N) -> Cookie
    where
        N: Into<Cow<'static, str>>,
    {
        Cookie::build(name, "").expires(Expires::remove()).build()
    }

    fn new_inner(name: TinyStr, value: TinyStr) -> Cookie {
        Cookie {
            name,
            value,
            ..Default::default()
        }
    }

    pub fn build<N, V>(name: N, value: V) -> CookieBuilder
    where
        N: Into<Cow<'static, str>>,
        V: Into<Cow<'static, str>>,
    {
        CookieBuilder::new(name, value)
    }

    #[inline]
    pub fn name(&self) -> &str {
        self.name.as_str(self.raw_value.as_deref())
    }

    #[inline]
    pub fn set_name<N: Into<Cow<'static, str>>>(&mut self, name: N) {
        self.name = TinyStr::from(name)
    }

    #[inline]
    pub fn value(&self) -> &str {
        self.value.as_str(self.raw_value.as_deref())
    }

    #[inline]
    pub fn set_value<V: Into<Cow<'static, str>>>(&mut self, value: V) {
        self.value = TinyStr::from(value)
    }

    #[inline]
    pub fn set_expires<E: Into<Expires>>(&mut self, expires: E) {
        self.expires = expires.into();
    }

    #[inline]
    pub fn max_age(&self) -> Option<Duration> {
        self.max_age.map(Duration::from_secs)
    }

    #[inline]
    pub fn max_age_secs(&self) -> Option<u64> {
        self.max_age
    }

    #[inline]
    pub fn set_max_age(&mut self, max_age: Duration) {
        self.set_max_age_secs(max_age.as_secs());
    }

    #[inline]
    pub fn set_max_age_secs(&mut self, max_age_secs: u64) {
        self.max_age = Some(max_age_secs);
    }

    #[inline]
    pub fn unset_max_age(&mut self) {
        self.max_age = None;
    }

    #[inline]
    pub fn domain(&self) -> Option<&str> {
        self.domain
            .as_ref()
            .map(|s| s.as_str(self.raw_value.as_deref()))
    }

    pub(crate) fn domain_sanitized(&self) -> Option<&str> {
        self.domain().map(|d| d.strip_prefix('.').unwrap_or(d))
    }

    #[inline]
    pub fn set_domain<D: Into<Cow<'static, str>>>(&mut self, domain: D) {
        self.domain = Some(TinyStr::from(domain))
    }

    #[inline]
    pub fn unset_domain(&mut self) {
        self.domain = None
    }

    #[inline]
    pub fn path(&self) -> Option<&str> {
        self.path
            .as_ref()
            .map(|val| val.as_str(self.raw_value.as_deref()))
    }

    #[inline]
    pub fn set_path<D: Into<Cow<'static, str>>>(&mut self, path: D) {
        self.path = Some(TinyStr::from(path))
    }

    #[inline]
    pub fn unset_path(&mut self) {
        self.path = None
    }

    #[inline]
    pub fn secure(&self) -> bool {
        self.secure
    }

    #[inline]
    pub fn set_secure(&mut self, secure: bool) {
        self.secure = secure
    }

    #[inline]
    pub fn http_only(&self) -> bool {
        self.http_only
    }

    #[inline]
    pub fn set_http_only(&mut self, http_only: bool) {
        self.http_only = http_only
    }

    #[inline]
    pub fn partitioned(&self) -> bool {
        self.partitioned
    }

    // Enabling the Partitioned attribute also enables the Secure attribute
    #[inline]
    pub fn set_partitioned(&mut self, partitioned: bool) {
        self.partitioned = partitioned;
    }

    #[inline]
    pub fn same_site(&self) -> Option<SameSite> {
        self.same_site
    }

    #[inline]
    pub fn set_same_site<S: Into<Option<SameSite>>>(&mut self, same_site: S) {
        self.same_site = same_site.into();
    }

    #[doc(hidden)]
    pub fn foo_bar() -> CookieBuilder {
        Cookie::build("foo", "bar")
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
            // .field("expires", &self.expires)
            .field("max_age", &self.max_age())
            .field("domain", &self.domain())
            .field("path", &self.path())
            .field("secure", &self.secure())
            .field("http_only", &self.http_only())
            .field("partitioned", &self.partitioned());

        #[cfg(feature = "time")]
        let debug = debug.field("expires_time", &self.expires_time());

        #[cfg(feature = "chrono")]
        let debug = debug.field("expires_chrono", &self.expires_chrono());

        #[cfg(feature = "jiff")]
        let debug = debug.field("expires_jiff", &self.expires_jiff());

        debug.finish()
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
        {
            return false;
        }

        // TODO: add support for expires here

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
