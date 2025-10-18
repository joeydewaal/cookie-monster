use crate::cookie::parse::find_invalid_cookie_value;

use super::Cookie;

impl Cookie {
    #[inline]
    pub(crate) fn serialize_domain(&self, buf: &mut String) {
        // Also removes the potential trailing dot.
        let Some(domain) = self.domain_sanitized() else {
            return;
        };

        // We skip empty domains.
        if domain.is_empty() {
            return;
        }

        // We're a bit conservative here and ignore domains that contain invalid cookie characters.
        // This makes the cookie a host-only cookie.
        if find_invalid_cookie_value(domain).is_some() {
            return;
        }

        buf.push_str("; Domain=");
        buf.push_str(domain);
    }
}
