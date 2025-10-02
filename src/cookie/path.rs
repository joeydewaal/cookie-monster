use crate::{Error, util::TinyStr};

use super::{Cookie, parse::invalid_cookie_value};

pub fn parse_path(path: &mut str, source: *const u8) -> Option<TinyStr> {
    if path.is_empty() {
        return None;
    }

    if !path.starts_with('/') {
        return None;
    }

    if path.chars().all(|char| !char.is_control() && char != ';') {
        Some(TinyStr::index(path, source))
    } else {
        None
    }
}

impl Cookie {
    #[inline]
    pub(crate) fn serialize_path(&self, buf: &mut String) -> crate::Result<()> {
        let Some(path) = self.path() else {
            return Ok(());
        };

        // We're more conservative here because a path attribute makes a cookie __more secure__.
        // Simply ignore the attribute could lead to some unexpected results.
        if path.is_empty() {
            return Err(Error::EmptyPathValue);
        }

        if !path.starts_with('/') || invalid_cookie_value(path) {
            return Err(Error::InvalidPathValue);
        }

        buf.push_str("; Path=");
        buf.push_str(path);
        Ok(())
    }
}
