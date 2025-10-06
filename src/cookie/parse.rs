use std::borrow::Cow;

use crate::SameSite;
use crate::{Cookie, error::Error, util::TinyStr};

use super::expires;
use super::{domain::parse_domain, max_age::parse_max_age, path::parse_path};

impl Cookie {
    pub fn parse_cookie(string: impl Into<Box<str>>) -> Result<Cookie, Error> {
        Self::parse_inner(
            string,
            |name, value| Ok((Cow::Borrowed(name), Cow::Borrowed(value))),
            false,
        )
    }

    pub fn parse_set_cookie(string: impl Into<Box<str>>) -> Result<Cookie, Error> {
        Self::parse_inner(
            string,
            |name, value| Ok((Cow::Borrowed(name), Cow::Borrowed(value))),
            true,
        )
    }

    #[cfg(feature = "percent-encode")]
    pub fn parse_cookie_encoded(string: impl Into<Box<str>>) -> Result<Cookie, Error> {
        use crate::cookie::encoding;

        Self::parse_inner(string, encoding::decode_name_value, false)
    }

    #[cfg(feature = "percent-encode")]
    pub fn parse_set_cookie_encoded(string: impl Into<Box<str>>) -> Result<Cookie, Error> {
        use crate::cookie::encoding;

        Self::parse_inner(string, encoding::decode_name_value, true)
    }

    fn parse_inner(
        string: impl Into<Box<str>>,
        callback: impl for<'a> Fn(&'a str, &'a str) -> crate::Result<(Cow<'a, str>, Cow<'a, str>)>,
        set_cookie: bool,
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

        if set_cookie {
            parse_attributes(&mut cookie, parts)?;
        }

        cookie.raw_value = Some(string);
        Ok(cookie)
    }
}

fn parse_attributes(cookie: &mut Cookie, mut parts: SplitMut) -> crate::Result<()> {
    // 1.  If the unparsed-attributes string is empty, skip the rest of
    // these steps.
    // 2.  Discard the first character of the unparsed-attributes (which
    //     will be a %x3B (";") character).

    while let Some(part) = parts.next() {
        // 4.  If the cookie-av string contains a %x3D ("=") character:
        let (mut name, value) = if let Some(index) = part.find('=') {
            // The (possibly empty) attribute-name string consists of the
            // characters up to, but not including, the first %x3D ("=")
            // character, and the (possibly empty) attribute-value string
            // consists of the characters after the first %x3D ("=")
            // character.
            let (name, mut value) = part.split_at_mut(index);

            // 5.  Remove any leading or trailing WSP characters from the attribute-
            //     name string and the attribute-value string.
            value = trim_mut(&mut value[1..]);

            (name, Some(value))
        } else {
            (part, None)
        };

        // 5.  Remove any leading or trailing WSP characters from the attribute-
        //     name string and the attribute-value string.
        name = trim_mut(name);
        name.make_ascii_lowercase();

        // 6.  Process the attribute-name and attribute-value according to the
        //     requirements in the following subsections.  (Notice that
        //     attributes with unrecognized attribute-names are ignored.)
        match (&*name, value) {
            ("secure", None) => cookie.set_secure(true),
            ("httponly", None) => cookie.set_http_only(true),
            ("partitioned", None) => cookie.set_partitioned(true),
            ("max-age", Some(value)) => cookie.max_age = parse_max_age(value),
            ("domain", Some(value)) => cookie.domain = parse_domain(value, parts.ptr),
            ("path", Some(value)) => cookie.path = parse_path(value, parts.ptr),
            ("expires", Some(value)) => cookie.expires = expires::parse_expires(value),
            ("samesite", Some(value)) => cookie.same_site = SameSite::from_attribute_value(value),
            _ => continue,
        }
    }
    Ok(())
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

pub fn trim_mut(mut str: &mut str) -> &mut str {
    let start = str.chars().take_while(|x| x.is_whitespace()).count();
    str = &mut str[start..];

    let end = str.chars().rev().take_while(|x| x.is_whitespace()).count();
    let end = str.len() - end;

    &mut str[..end]
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

#[cfg(test)]
mod tests {
    use super::SplitMut;
    use crate::cookie::parse::trim_mut;

    #[test]
    fn test_split() {
        let mut haystack = "test;test;".to_string();
        let split = SplitMut::new(&mut haystack);

        let found: Vec<&mut str> = split.collect();
        assert!(found == ["test", "test", ""]);
    }

    #[test]
    fn test_trim_mut() {
        let expect_trim = |input: &'static str| {
            let mut h = input.to_string();
            assert_eq!(trim_mut(&mut h), "HelloWorld");
        };

        expect_trim(" HelloWorld");
        expect_trim("  HelloWorld");
        expect_trim("HelloWorld ");
        expect_trim("HelloWorld  ");
        expect_trim(" HelloWorld ");
        expect_trim("\nHelloWorld\n");
        expect_trim("\n\tHelloWorld\t\n");
    }
}
