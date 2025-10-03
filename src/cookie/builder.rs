use std::{
    borrow::{Borrow, Cow},
    fmt,
    time::Duration,
};

use crate::Cookie;

use super::{expires::Expires, same_site::SameSite};

#[derive(PartialEq)]
pub struct CookieBuilder(Cookie);

impl CookieBuilder {
    #[inline]
    pub fn new<N, V>(name: N, value: V) -> CookieBuilder
    where
        N: Into<Cow<'static, str>>,
        V: Into<Cow<'static, str>>,
    {
        CookieBuilder(Cookie::new(name, value))
    }

    // Sets the name of the cookie.
    #[inline]
    pub fn name<N: Into<Cow<'static, str>>>(mut self, name: N) -> Self {
        self.0.set_name(name);
        self
    }

    // Sets the value of the cookie.
    #[inline]
    pub fn value<V: Into<Cow<'static, str>>>(mut self, value: V) -> Self {
        self.0.set_value(value);
        self
    }

    #[inline]
    pub fn expires(mut self, expiration: impl Into<Expires>) -> Self {
        self.0.set_expires(expiration.into());
        self
    }

    // Sets the Max-Age attribute of the cookie.
    #[inline]
    pub fn max_age(mut self, max_age: Duration) -> Self {
        self.0.set_max_age(max_age);
        self
    }

    // Sets the Domain attribute of the cookie.
    #[inline]
    pub fn domain<D: Into<Cow<'static, str>>>(mut self, domain: D) -> Self {
        self.0.set_domain(domain);
        self
    }

    // Sets the Path attribute of the cookie.
    #[inline]
    pub fn path<D: Into<Cow<'static, str>>>(mut self, path: D) -> Self {
        self.0.set_path(path);
        self
    }

    // Sets the Secure attribute of the cookie.
    #[inline]
    pub fn secure(mut self) -> Self {
        self.0.set_secure(true);
        self
    }

    #[inline]
    pub fn set_secure(mut self, secure: bool) -> Self {
        self.0.set_secure(secure);
        self
    }

    // Sets the HttpOnly attribute of the cookie.
    #[inline]
    pub fn http_only(mut self) -> Self {
        self.0.set_http_only(true);
        self
    }

    #[inline]
    pub fn set_http_only(mut self, http_only: bool) -> Self {
        self.0.set_http_only(http_only);
        self
    }

    // Sets the Partitioned attribute of the cookie. Enabling the Partitioned attribute also
    // enables the Secure attribute.
    //
    // To disable it use `set_partitioned(false)` TODO
    //
    // https://developer.mozilla.org/en-US/docs/Web/Privacy/Guides/Privacy_sandbox/Partitioned_cookies
    #[inline]
    pub fn partitioned(self) -> Self {
        self.set_partitioned(true)
    }

    #[inline]
    pub fn set_partitioned(mut self, partitioned: bool) -> Self {
        self.0.set_partitioned(partitioned);
        self
    }

    #[inline]
    pub fn same_site<S: Into<Option<SameSite>>>(mut self, same_site: S) -> Self {
        self.0.set_same_site(same_site);
        self
    }

    // Builds and returns the cookie
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
