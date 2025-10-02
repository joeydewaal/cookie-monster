use crate::util::TinyStr;

use super::{Cookie, parse::invalid_cookie_value};

pub fn parse_domain(domain: &mut str, src: *const u8) -> Option<TinyStr> {
    // If the attribute-value is empty, the behavior is undefined.  However,
    // the user agent SHOULD ignore the cookie-av entirely.
    // (We are permissive here and just ignore the domain and go on with parsing.)
    if domain.is_empty() {
        return None;
    }

    domain.make_ascii_lowercase();
    let mut domain = &*domain;

    // We're conservative here and don't allow invalid cookie characters. If you think we
    // should, please open an issue.
    if invalid_cookie_value(domain) {
        return None;
    }

    // If the first character of the attribute-value string is %x2E ("."):
    //    Let cookie-domain be the attribute-value without the leading %x2E
    //    (".") character.
    // Otherwise:
    //    Let cookie-domain be the entire attribute-value.
    if domain.starts_with('.') {
        domain = &domain[1..];
    }

    // Convert the cookie-domain to lower case.
    // (attribute is always going to be lowercase)

    // Append an attribute to the cookie-attribute-list with an attribute-
    // name of Domain and an attribute-value of cookie-domain.
    Some(TinyStr::index(domain, src))
}

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
        if invalid_cookie_value(domain) {
            return;
        }

        buf.push_str("; Domain=");
        buf.push_str(domain);
    }
}
