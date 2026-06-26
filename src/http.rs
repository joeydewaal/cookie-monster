use http::HeaderMap;

use crate::{Cookie, CookieJar};

impl CookieJar {
    /// Builds a `CookieJar` from the `Cookie` request headers, percent-decoding
    /// names and values and ignoring cookies that fail to parse.
    ///
    /// Duplicate cookie names resolve to the **last** occurrence; see
    /// [`from_cookie`](CookieJar::from_cookie) for the cookie-shadowing note.
    pub fn from_headers(headers: &HeaderMap) -> Self {
        let iter = headers
            .get_all("cookie")
            .into_iter()
            .filter_map(|header| header.to_str().ok())
            .flat_map(|cookie_str| cookie_str.split(';'))
            .filter_map(|string| Cookie::parse_cookie_encoded(string).ok());

        CookieJar::from_original(iter)
    }

    pub fn write_cookies(self, headers: &mut HeaderMap) {
        for cookie in self.iter_non_original() {
            if let Some(header) = cookie
                .serialize_encoded()
                .ok()
                .and_then(|string| string.parse().ok())
            {
                headers.append("set-cookie", header);
            }
        }
    }
}
