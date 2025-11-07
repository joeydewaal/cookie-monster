use std::borrow::Cow;

use crate::{Cookie, error::Error, util::TinyStr};

impl Cookie {
    /// Parses the given cookie header value. Errors when:
    /// * No '=' is found.
    /// * The name is empty.
    /// * The name contains an invalid character.
    /// * The cookie value contains an invalid character.
    ///
    /// Since this only parses a cookie header value, it does not parse any cookie attributes.
    pub fn parse_cookie(string: impl Into<String>) -> crate::Result<Cookie> {
        Self::parse_inner(string.into(), |name, value| {
            Ok((Cow::Borrowed(name), Cow::Borrowed(value)))
        })
    }

    /// Parses a percent encoded cookie value. Errors when:
    /// * No '=' is found.
    /// * The name is empty.
    /// * The name contains an invalid character.
    /// * The cookie value contains an invalid character.
    ///
    /// Since this only parses a cookie header value, it does not parse any cookie attributes.
    #[cfg(feature = "percent-encode")]
    pub fn parse_cookie_encoded(string: impl Into<String>) -> crate::Result<Cookie> {
        use crate::cookie::encoding;

        Self::parse_inner(string.into(), encoding::decode_name_value)
    }

    fn parse_inner(
        mut string: String,
        callback: impl for<'a> Fn(&'a str, &'a str) -> crate::Result<(Cow<'a, str>, Cow<'a, str>)>,
    ) -> Result<Cookie, Error> {
        let mut parts = SplitMut::new(&mut string);

        let name_value = parts.next().expect("First split always returns something");

        // 2.  If the name-value-pair string lacks a %x3D ("=") character,
        //     ignore the set-cookie-string entirely.
        let Some(index) = name_value.find('=') else {
            return Err(Error::EqualsNotFound);
        };

        // 4.  Remove any leading or trailing WSP characters from the name
        //     string and the value string.
        let name = name_value[..index].trim();
        let mut value = name_value[(index + 1)..].trim();

        // 5.  If the name string is empty, ignore the set-cookie-string entirely.
        if name.is_empty() {
            return Err(Error::NameEmpty);
        } else if !is_token(name) {
            return Err(Error::InvalidName);
        }

        // Remove optional brackets.
        value = trim_quotes(value);

        if let Some(invalid_char) = find_invalid_cookie_value(value) {
            return Err(Error::InvalidValue(invalid_char));
        }

        let (name, value) = callback(name, value)?;

        let name = TinyStr::from_cow_ref(name, parts.ptr);
        let value = TinyStr::from_cow_ref(value, parts.ptr);

        let mut cookie = Cookie::new_inner(name, value);
        cookie.raw_value = Some(string);
        Ok(cookie)
    }
}

struct SplitMut<'s> {
    haystack: Option<&'s mut str>,
    ptr: *const u8,
}

impl<'s> SplitMut<'s> {
    pub fn new(haystack: &'s mut str) -> SplitMut<'s> {
        SplitMut {
            ptr: haystack.as_ptr(),
            haystack: Some(haystack),
        }
    }
}

impl<'s> Iterator for SplitMut<'s> {
    type Item = &'s mut str;

    fn next(&mut self) -> Option<Self::Item> {
        let remainder = self.haystack.take()?;

        match remainder.find(';') {
            Some(index) => {
                let (current, rest) = remainder.split_at_mut(index);

                if !rest.is_empty() {
                    self.haystack = Some(&mut rest[1..]);
                } else {
                    self.haystack = Some(&mut rest[..]);
                }

                Some(current)
            }
            None => Some(remainder),
        }
    }
}

#[inline]
fn invalid_cookie_value_char(val: &char) -> bool {
    match val {
        ' ' | '"' | ',' | ';' | '\\' => true,
        control if control.is_ascii_control() => true,
        _ => false,
    }
}

pub fn trim_quotes(value: &str) -> &str {
    if value.len() > 1 && value.starts_with('"') && value.ends_with('"') {
        &value[1..(value.len() - 1)]
    } else {
        value
    }
}

#[inline]
pub fn is_token(val: &str) -> bool {
    val.chars().all(|c| match c {
        '!' | '#' | '$' | '%' | '&' | '\'' | '*' | '+' | '-' | '.' | '^' | '_' | '`' | '|'
        | '~' => true,
        c if c.is_alphanumeric() => true,
        _ => false,
    })
}

#[inline]
pub fn find_invalid_cookie_value(val: &str) -> Option<char> {
    val.chars().find(invalid_cookie_value_char)
}
