use crate::{Error, cookie::parse::find_invalid_cookie_value};

use super::Cookie;

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

        if !path.starts_with('/') {
            // TODO
            return Err(Error::NoLeadingSlash);
        } else if let Some(invalid_char) = find_invalid_cookie_value(path) {
            return Err(Error::InvalidPathValue(invalid_char));
        }

        buf.push_str("; Path=");
        buf.push_str(path);
        Ok(())
    }
}
