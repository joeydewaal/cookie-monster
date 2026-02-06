use http::HeaderMap;

use crate::{Cookie, CookieJar};

impl CookieJar {
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

#[cfg(test)]
mod axum_tests {
    use axum::response::IntoResponse;

    use crate::Cookie;

    #[test]
    fn set_multiple_cookies() {
        let first = Cookie::build("foo", "bar").build();
        let second = Cookie::build("baz", "qux").build();

        let response = (first, ()).into_response();

        assert!(response.headers().get_all("set-cookie").iter().count() == 1);

        let response = (second, response).into_response();
        assert!(response.headers().get_all("set-cookie").iter().count() == 2);
    }
}
