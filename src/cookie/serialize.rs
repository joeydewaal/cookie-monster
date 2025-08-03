use crate::Error;

use super::{Cookie, parse::invalid_cookie_value};
use std::fmt::Write;

impl Cookie {
    pub fn serialize(&self) -> crate::Result<String> {
        self.serialize_inner(false)
    }

    pub fn serialize_unchecked(&self) -> String {
        self.serialize_inner(true)
            .expect("Unchecked serialize should not return an error")
    }

    fn serialize_inner(&self, is_unchecked: bool) -> crate::Result<String> {
        let value = self.value();
        let name = self.name();
        let domain = self.domain();
        let path = self.path();

        if !is_unchecked {
            if name.is_empty() {
                return Err(Error::NameEmpty);
            } else if invalid_cookie_value(name) {
                return Err(Error::InvalidName);
            }

            let value_to_check =
                if value.len() > 1 && value.starts_with('"') && value.ends_with('"') {
                    &value[1..(value.len() - 1)]
                } else {
                    value
                };

            if invalid_cookie_value(value_to_check) {
                return Err(Error::InvalidValue);
            }
        }

        let buf_len = name.len()
            + value.len()
            + domain.map(str::len).unwrap_or_default()
            + path.map(str::len).unwrap_or_default();

        let mut buf = String::with_capacity(buf_len + 110);

        buf.push_str(name);
        buf.push('=');
        buf.push_str(value);

        // Expires

        if let Some(max_age) = self.max_age_secs() {
            buf.push_str("; Max-Age=");
            write!(&mut buf, "{max_age}").expect("Failed to write Max-Age seconds");
        }

        self.serialize_domain(&mut buf, is_unchecked)?;

        self.serialize_path(&mut buf, is_unchecked)?;

        // Partitioned cookies need the Secure attribute
        if self.secure() || self.partitioned() {
            buf.push_str("; Secure");
        }

        if self.http_only() {
            buf.push_str("; HttpOnly");
        }

        if self.partitioned() {
            buf.push_str("; Partitioned");
        }

        self.serialize_expire(&mut buf)?;
        Ok(buf)
    }
}
