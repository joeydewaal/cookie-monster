use std::{borrow::Borrow, collections::HashSet, fmt::Debug, hash::Hash};

use crate::Cookie;

/// A generic `CookieJar` for cookie management. Can be used to read update or delete cookies from
/// a user session.
///
/// ## `axum` feature
///
/// Note that to set the cookies, the jar _must_ be returned from the handler. Otherwise the
/// cookies are not updated.
///
///
/// ## Example
/// ```rust
/// use cookie_monster::{CookieJar, Cookie};
///
/// static COOKIE_NAME: &str = "session";
///
/// async fn handler(mut jar: CookieJar) -> CookieJar {
///
///     if let Some(cookie) = jar.get(COOKIE_NAME) {
///         println!("Removing cookie {cookie:?}");
///         jar.remove(Cookie::named(COOKIE_NAME));
///     } else {
///         let cookie = Cookie::new(COOKIE_NAME, "hello, world");
///         println!("Setting cookie {cookie:?}");
///         jar.add(cookie);
///     }
///
///     // Important, return the jar to update the cookies!
///     jar
/// }
/// ```
#[derive(Default, Debug)]
pub struct CookieJar {
    cookies: HashSet<HashCookie>,
}

pub(crate) enum HashCookie {
    // An original cookie. These should never be sent back to the user-agent.
    Original(Cookie),
    // A new cookie, the should always be sent back to the user-agent.
    New(Cookie),
    // A removed cookie, the should always be sent back to the user-agent but should never be
    // visible by the user.
    Removal(Cookie),
}

impl HashCookie {
    fn name(&self) -> &str {
        match self {
            HashCookie::Original(c) => c.name(),
            HashCookie::New(c) => c.name(),
            HashCookie::Removal(c) => c.name(),
        }
    }
}

impl Hash for HashCookie {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name().hash(state);
    }
}

impl PartialEq for HashCookie {
    fn eq(&self, other: &Self) -> bool {
        self.name() == other.name()
    }
}

impl Borrow<str> for HashCookie {
    fn borrow(&self) -> &str {
        self.name()
    }
}

impl Eq for HashCookie {}

impl Debug for HashCookie {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HashCookie::Original(cookie) => cookie.fmt(f),
            HashCookie::New(cookie) => cookie.fmt(f),
            HashCookie::Removal(cookie) => cookie.fmt(f),
        }
    }
}

impl CookieJar {
    /// Creates an empty `CookieJar`.
    pub fn empty() -> Self {
        Self::default()
    }

    /// Parses the given `cookie` header value and return a `CookieJar`. This function ignores
    /// cookies that were not able to be parsed.
    pub fn from_cookie(header: &str) -> Self {
        Self::from_original(header.split(';').flat_map(Cookie::parse_cookie))
    }

    /// Parses the given `cookie` header value and return a `CookieJar`. The cookie name and values
    /// are percent-decoded. Cookies that were not able to be parsed are ignored.
    #[cfg(feature = "percent-encode")]
    pub fn from_encoded_cookie(header: &str) -> Self {
        Self::from_original(header.split(';').flat_map(Cookie::parse_cookie_encoded))
    }

    /// Adds an __original__ cookie to the jar. These are never sent back to the
    /// user-agent, but are visible in the cookie jar.
    pub fn add_original(&mut self, cookie: Cookie) {
        self.cookies.insert(HashCookie::Original(cookie));
    }

    // Creates a `CookieJar` from an iterator of cookies. It is assumed that the cookies are
    // __original__. E.g. from a `Cookie` header value.
    pub fn from_original<T: IntoIterator<Item = Cookie>>(cookies: T) -> Self {
        let mut jar = Self::empty();

        for cookie in cookies {
            jar.add_original(cookie);
        }

        jar
    }

    /// Get a cookie by name. Gives back either an __original__ or newly added cookie.
    pub fn get(&self, name: &str) -> Option<&Cookie> {
        self.cookies.get(name).and_then(|c| match c {
            HashCookie::New(c) | HashCookie::Original(c) => Some(c),
            HashCookie::Removal(_) => None,
        })
    }

    /// Iterate over all changes. This returns all removed and newly created cookies.
    pub fn set_cookie_headers(&self) -> impl Iterator<Item = crate::Result<String>> {
        self.cookies
            .iter()
            .filter_map(|c| match c {
                HashCookie::Original(_) => None,
                HashCookie::New(c) | HashCookie::Removal(c) => Some(c),
            })
            .map(Cookie::serialize)
    }

    /// Removes the cookie from the local cookie store and issues a cookie with an Expires
    /// attribute in the past and Max-Age of 0 seconds.
    ///
    /// **To ensure a cookie is removed from the user-agent, set the `Path` and `Domain` attributes
    /// with the same values that were used to create the cookie.**
    pub fn remove(&mut self, cookie: impl Into<Cookie>) {
        let cookie = HashCookie::Removal(cookie.into().into_remove());
        self.cookies.replace(cookie);
    }

    /// Adds a cookie to the jar. If a cookie with the same name is already in the jar, it is
    /// replaced with the given cookie.
    pub fn add(&mut self, cookie: Cookie) {
        self.cookies.replace(HashCookie::New(cookie));
    }

    #[allow(unused)]
    pub(crate) fn iter_non_original(&self) -> impl Iterator<Item = &Cookie> {
        self.cookies.iter().flat_map(|cookie| match cookie {
            HashCookie::Original(_) => None,
            HashCookie::New(cookie) | HashCookie::Removal(cookie) => Some(cookie),
        })
    }
}
