use crate::cookie::parse::is_valid_cookie_value;

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
        if !is_valid_cookie_value(domain) {
            return;
        }

        buf.push_str("; Domain=");
        buf.push_str(domain);
    }
}
