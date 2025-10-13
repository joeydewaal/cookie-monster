use http::HeaderMap;

use crate::{Cookie, CookieJar};

impl CookieJar {
    pub fn from_headers(headers: &HeaderMap) -> Self {
        let iter = headers
            .get_all("cookie")
            .into_iter()
            .flat_map(|header| header.to_str().ok())
            .flat_map(|string| Cookie::parse_cookie_encoded(string).ok());

        CookieJar::from_original(iter)
    }

    pub fn write_cookies(&self, headers: &mut HeaderMap) {
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
