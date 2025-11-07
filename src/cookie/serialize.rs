use crate::{
    Error, SameSite,
    cookie::parse::{find_invalid_cookie_value, is_token, trim_quotes},
};

use super::Cookie;
use std::fmt::Write;

impl Cookie {
    // Serializes the cookie. Errors when:
    // * The name is empty.
    // * Name or value contain invalid cookie character.
    // * Path attribute is empty.
    // * Path attribute does not start with a leading '/'.
    // * Path attribute contains an invalid cookie character.
    //
    // Ignores domains with invalid cookie characters.
    pub fn serialize(&self) -> crate::Result<String> {
        self.serialize_inner(|name, value, buf| {
            // Unencdoded values need manual validation of the characters.

            let trimmed_value = trim_quotes(value);

            if let Some(invalid_char) = find_invalid_cookie_value(trimmed_value) {
                return Err(Error::InvalidValue(invalid_char));
            } else if !is_token(name) {
                return Err(Error::InvalidName);
            }

            let _ = write!(buf, "{name}={value}");
            Ok(())
        })
    }

    // Serializes and percent encodes the cookie. Errors when:
    // * The name is empty.
    // * Path attribute is empty.
    // * Path attribute does not start with a leading '/'.
    // * Path attribute contains an invalid cookie character.
    //
    // Ignores domains with invalid cookie characters.
    #[cfg(feature = "percent-encode")]
    pub fn serialize_encoded(&self) -> crate::Result<String> {
        use crate::cookie::encoding::{encode_name, encode_value};

        self.serialize_inner(|name, value, buf| {
            // Encoded values don't require validation since the invalid characters are encoded.
            let _ = write!(buf, "{}={}", encode_name(name), encode_value(value));
            Ok(())
        })
    }

    fn serialize_inner(
        &self,
        callback: impl Fn(&str, &str, &mut String) -> crate::Result<()>,
    ) -> crate::Result<String> {
        let value = self.value();
        let name = self.name();
        let domain = self.domain();
        let path = self.path();

        if name.is_empty() {
            return Err(Error::NameEmpty);
        }

        let buf_len = name.len()
            + value.len()
            + domain.map(str::len).unwrap_or_default()
            + path.map(str::len).unwrap_or_default();

        // 110 is derived from typical length of cookie attributes
        // see RFC 6265 Sec 4.1.
        let mut buf = String::with_capacity(buf_len + 110);

        // Write name and value
        // Validation happens in the callback.
        callback(name, value, &mut buf)?;

        // Expires
        if let Some(max_age) = self.max_age_secs() {
            buf.push_str("; Max-Age=");
            write!(&mut buf, "{max_age}").expect("Failed to write Max-Age seconds");
        }

        self.serialize_domain(&mut buf);

        self.serialize_path(&mut buf)?;

        // SameSite=None and Partitioned cookies need the Secure attribute
        if self.secure() || self.partitioned() || self.same_site() == Some(SameSite::None) {
            buf.push_str("; Secure");
        }

        if self.http_only() {
            buf.push_str("; HttpOnly");
        }

        if self.partitioned() {
            buf.push_str("; Partitioned");
        }

        self.serialize_same_site(&mut buf);

        self.serialize_expire(&mut buf)?;
        Ok(buf)
    }
}
