use std::borrow::Cow;

use crate::{Cookie, error::Error, util::TinyStr};

impl Cookie {
    pub fn parse_cookie(string: impl Into<Box<str>>) -> Result<Cookie, Error> {
        Self::parse_inner(string, |name, value| {
            Ok((Cow::Borrowed(name), Cow::Borrowed(value)))
        })
    }

    #[cfg(feature = "percent-encode")]
    pub fn parse_cookie_encoded(string: impl Into<Box<str>>) -> Result<Cookie, Error> {
        use crate::cookie::encoding;

        Self::parse_inner(string, encoding::decode_name_value)
    }

    fn parse_inner(
        string: impl Into<Box<str>>,
        callback: impl for<'a> Fn(&'a str, &'a str) -> crate::Result<(Cow<'a, str>, Cow<'a, str>)>,
    ) -> Result<Cookie, Error> {
        let mut string = string.into();
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

        if !is_valid_cookie_value(value) {
            return Err(Error::InvalidValue);
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
fn allowed_cookie_value(val: char) -> bool {
    match val {
        ' ' | '"' | ',' | ';' | '\\' => false,
        control if control.is_ascii_control() => false,
        _ => true,
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
pub fn is_valid_cookie_value(val: &str) -> bool {
    val.chars().all(allowed_cookie_value)
}
