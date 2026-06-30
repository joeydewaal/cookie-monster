use std::{borrow::Borrow, collections::HashSet, fmt::Debug, hash::Hash};

use crate::{
    Cookie,
    cookie::prefix::{HOST_PREFIX, SECURE_PREFIX},
};

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
    pub fn new() -> Self {
        Self::default()
    }

    /// Parses the given `cookie` header value and return a `CookieJar`. This function ignores
    /// cookies that were not able to be parsed.
    ///
    /// # Duplicate names
    /// If the header contains the same cookie name more than once, the **last**
    /// occurrence wins. This matches the `cookie` crate, `axum-extra`, Python's
    /// `SimpleCookie` and ASP.NET Core.
    ///
    /// Duplicate-name resolution is **not** a security boundary. To defend
    /// against cookie shadowing/tossing, use `__Host-`/`__Secure-` name prefixes
    /// and/or reject requests that carry duplicate cookie names.
    ///
    /// ```rust
    /// use cookie_monster::CookieJar;
    ///
    /// let jar = CookieJar::from_cookie("name=first; name=second");
    /// assert_eq!(jar.get("name").map(|c| c.value()), Some("second"));
    /// ```
    pub fn from_cookie(header: &str) -> Self {
        Self::from_original(header.split(';').flat_map(Cookie::parse_cookie))
    }

    /// Parses the given `cookie` header value and return a `CookieJar`. The cookie name and values
    /// are percent-decoded. Cookies that were not able to be parsed are ignored.
    ///
    /// Like [`from_cookie`](Self::from_cookie), duplicate cookie names resolve to
    /// the **last** occurrence; see its docs for the cookie-shadowing note.
    #[cfg(feature = "percent-encode")]
    pub fn from_encoded_cookie(header: &str) -> Self {
        Self::from_original(header.split(';').flat_map(Cookie::parse_cookie_encoded))
    }

    /// Adds an __original__ cookie to the jar. These are never sent back to the
    /// user-agent, but are visible in the cookie jar.
    ///
    /// If a cookie with the same name is already present it is replaced
    /// (last-wins), matching [`add`](Self::add).
    pub fn add_original(&mut self, cookie: Cookie) {
        self.cookies.replace(HashCookie::Original(cookie));
    }

    // Creates a `CookieJar` from an iterator of cookies. It is assumed that the cookies are
    // __original__. E.g. from a `Cookie` header value.
    pub fn from_original<T: IntoIterator<Item = Cookie>>(cookies: T) -> Self {
        let mut jar = Self::new();

        for cookie in cookies {
            jar.add_original(cookie);
        }

        jar
    }

    /// Get a cookie by name. Gives back either an __original__ or newly added cookie.
    ///
    /// `name` may be given without the `__Host-` / `__Secure-` prefix: a prefixed cookie of
    /// the same logical name is preferred over a non-prefixed one (`__Host-` over `__Secure-`
    /// over no prefix). This means a plain `id` cookie can never shadow a `__Host-id` cookie,
    /// and a cookie set with [`Cookie::host`] / [`Cookie::secure`] can be read back by its
    /// unprefixed name. The full prefixed name resolves as well.
    pub fn get(&self, name: &str) -> Option<&Cookie> {
        self.resolve(name).and_then(|c| match c {
            HashCookie::New(c) | HashCookie::Original(c) => Some(c),
            HashCookie::Removal(_) => None,
        })
    }

    /// Resolves a (possibly unprefixed) name to a stored cookie, preferring the `__Host-`
    /// then `__Secure-` prefixed variant before falling back to an exact match.
    fn resolve(&self, name: &str) -> Option<&HashCookie> {
        self.cookies
            .get(format!("{HOST_PREFIX}{name}").as_str())
            .or_else(|| self.cookies.get(format!("{SECURE_PREFIX}{name}").as_str()))
            .or_else(|| self.cookies.get(name))
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
    /// If one of the `time`, `chrono` or `jiff` features are enabled, the Expires tag is set to the
    /// current time minus one year. If none of the those features are enabled, the Expires
    /// attribute is set to 1 Jan 1970 00:00.
    ///
    /// **To ensure a cookie is removed from the user-agent, set the `Path` and `Domain` attributes
    /// with the same values that were used to create the cookie.**
    pub fn remove(&mut self, cookie: impl Into<Cookie>) -> Option<Cookie> {
        let cookie = HashCookie::Removal(cookie.into().into_remove());
        self.cookies.replace(cookie).and_then(|c| match c {
            HashCookie::Original(cookie) => Some(cookie),
            HashCookie::New(cookie) => Some(cookie),
            HashCookie::Removal(_) => None,
        })
    }

    /// Adds a cookie to the jar. If a cookie with the same name is already in the jar, it is
    /// replaced with the given cookie.
    pub fn add(&mut self, cookie: impl Into<Cookie>) {
        self.cookies.replace(HashCookie::New(cookie.into()));
    }

    #[allow(unused)]
    pub(crate) fn iter_non_original(&self) -> impl Iterator<Item = &Cookie> {
        self.cookies.iter().flat_map(|cookie| match cookie {
            HashCookie::Original(_) => None,
            HashCookie::New(cookie) | HashCookie::Removal(cookie) => Some(cookie),
        })
    }
}
