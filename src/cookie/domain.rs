use crate::util::TinyStr;

use super::{Cookie, parse::invalid_cookie_value};

pub fn parse_domain(domain: &mut str, src: *const u8, is_unchecked: bool) -> Option<TinyStr> {
    domain.make_ascii_lowercase();

    if !is_unchecked && invalid_cookie_value(domain) {
        return None;
    }

    let mut domain = &*domain;
    // If the attribute-value is empty, the behavior is undefined.  However,
    // the user agent SHOULD ignore the cookie-av entirely.
    if domain.is_empty() {
        if is_unchecked {
            return Some(TinyStr::empty());
        } else {
            return None;
        }
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
    pub(crate) fn serialize_domain(
        &self,
        buf: &mut String,
        is_unchecked: bool,
    ) -> crate::Result<()> {
        let Some(mut domain) = self.domain() else {
            return Ok(());
        };

        if is_unchecked {
            write_domain(buf, domain);
            return Ok(());
        }

        // We skip empty domains.
        if domain.is_empty() {
            return Ok(());
        }

        if domain.starts_with('.') {
            domain = &domain[1..];
        }

        if invalid_cookie_value(domain) {
            return Ok(());
        }

        write_domain(buf, domain);
        Ok(())
    }
}

fn write_domain(buf: &mut String, domain: &str) {
    buf.push_str("; Domain=");
    buf.push_str(domain);
}
