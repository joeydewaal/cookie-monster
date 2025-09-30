use crate::{Error, util::TinyStr};

use super::{Cookie, parse::invalid_cookie_value};

pub fn parse_path(path: &mut str, source: *const u8) -> Option<TinyStr> {
    // If the attribute-value is empty or if the first character of the
    //     attribute-value is not %x2F ("/"):
    //     Let cookie-path be the default-path.
    // Otherwise:
    //    Let cookie-path be the attribute-value.
    if !valid_path(path) {
        None
    } else {
        Some(TinyStr::index(path, source))
    }
}

#[inline]
pub fn valid_path(path: &str) -> bool {
    if path.is_empty() {
        return false;
    }

    if !path.starts_with('/') {
        return false;
    }

    path.chars().all(|char| !char.is_control() && char != ';')
}

impl Cookie {
    #[inline]
    pub(crate) fn serialize_path(&self, buf: &mut String, is_unchecked: bool) -> crate::Result<()> {
        let Some(path) = self.path() else {
            return Ok(());
        };

        if is_unchecked {
            write_path(buf, path);
            return Ok(());
        }

        if path.is_empty() {
            return Ok(());
        }

        if !path.starts_with('/') || invalid_cookie_value(path) {
            return Err(Error::InvalidAttribute);
        }

        write_path(buf, path);
        Ok(())
    }
}

fn write_path(buf: &mut String, path: &str) {
    buf.push_str("; Path=");
    buf.push_str(path);
}
