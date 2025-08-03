use crate::{Cookie, error::Error, util::TinyStr};

use super::expires;
use super::{domain::parse_domain, max_age::parse_max_age, path::parse_path};

use super::options::CookieOptions;

impl Cookie {
    /// Follows RFC 6265 thoroughly, this means:
    /// Characters that are not allowed in the name, value or attributes (e.g. ascii control
    /// characters and spaces...) return an error.
    ///
    /// *Expires*
    ///
    /// *Max-Age*
    /// if a '+' is found in the value the attribute value is skipped
    ///
    /// *Domain*
    /// An empty domain is ignored.
    ///
    /// *Path*
    /// An empty path is ignored.
    ///
    /// *Secure,HttpOnly,Partitioned*
    pub fn parse_strict(string: impl Into<Box<str>>) -> Result<Cookie, Error> {
        Self::parse_inner(string.into(), CookieOptions::strict())
    }

    /// Follows RFC 6265  but less strict, this means:
    /// Characters that are not allowed in the name, value(e.g. ascii control characters and
    /// spaces...) return an error. But invalid characters in attribute name/values are ignored.
    ///
    /// *Expires*
    ///
    /// *Max-Age*
    /// if a '+' is found the max-age value is still parsed.
    ///
    /// *Domain*
    /// An empty domain is ignored.
    ///
    /// *Path*
    /// An empty path is ignored.
    ///
    /// *Secure,HttpOnly,Partitioned*
    pub fn parse_relaxed(string: impl Into<Box<str>>) -> Result<Cookie, Error> {
        Self::parse_inner(string.into(), CookieOptions::relaxed())
    }
    /// Mostly follows RFC 6265  but does not check for invalid characters:
    /// Characters that are not allowed in the name, value(e.g. ascii control characters and
    /// spaces...) return an error. But invalid characters in attribute name/values are ignored.
    /// If no '=' is found, an empty cookie is returned.
    ///
    /// *Expires*
    ///
    /// *Max-Age*
    /// if a '+' is found the max-age value is still parsed.
    ///
    /// *Domain*
    /// An empty domain counts as an empty domain value.
    ///
    /// *Path*
    /// An empty path counts as an empty path value.
    ///
    /// *Secure,HttpOnly,Partitioned*
    pub fn parse_unchecked(string: impl Into<Box<str>>) -> Cookie {
        Self::parse_inner(string.into(), CookieOptions::unchecked())
            .expect("unchecked should never panic")
    }

    fn parse_inner(mut string: Box<str>, options: CookieOptions) -> Result<Cookie, Error> {
        let is_unchecked = options.is_unchecked();

        let mut parts = SplitMut::new(&mut string);

        let name_value = parts.next().expect("First split always returns something");

        // 2.  If the name-value-pair string lacks a %x3D ("=") character,
        //     ignore the set-cookie-string entirely.
        let Some(index) = name_value.find('=') else {
            if is_unchecked {
                return Ok(Cookie::empty());
            } else {
                return Err(Error::EqualsNotFound);
            }
        };

        // 4.  Remove any leading or trailing WSP characters from the name
        //     string and the value string.
        let name = name_value[..index].trim();
        let mut value = name_value[(index + 1)..].trim();

        // 5.  If the name string is empty, ignore the set-cookie-string entirely.
        if !is_unchecked && name.is_empty() {
            return Err(Error::NameEmpty);
        } else if !is_unchecked && invalid_cookie_value(name) {
            return Err(Error::InvalidName);
        }

        // Remove optional brackets.
        if value.len() > 1 && value.starts_with('"') && value.ends_with('"') {
            value = &value[1..(value.len() - 1)];
        }

        if invalid_cookie_value(value) && !is_unchecked {
            return Err(Error::InvalidValue);
        }

        let name = TinyStr::index(name, parts.ptr);
        let value = TinyStr::index(value, parts.ptr);

        let mut cookie = Cookie::new_inner(name, value);

        parse_attributes(&mut cookie, parts, options)?;
        cookie.raw_value = Some(string);
        Ok(cookie)
    }
}

fn parse_attributes(
    cookie: &mut Cookie,
    mut parts: SplitMut,
    options: CookieOptions,
) -> crate::Result<()> {
    let is_strict = options.is_strict();

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

            // match options.strictness() {
            //     Strictness::Strict if invalid_cookie_value(value) => {
            //         return Err(Error::InvalidAttribute);
            //     }
            //     Strictness::Relaxed if invalid_cookie_value(value) => continue,
            //     _ => {}
            // }

            (name, Some(value))
        } else {
            (part, None)
        };

        // 5.  Remove any leading or trailing WSP characters from the attribute-
        //     name string and the attribute-value string.
        name = trim_mut(name);
        name.make_ascii_lowercase();

        if is_strict && invalid_cookie_value(name) {
            return Err(Error::InvalidAttribute);
        }

        // 6.  Process the attribute-name and attribute-value according to the
        //     requirements in the following subsections.  (Notice that
        //     attributes with unrecognized attribute-names are ignored.)
        match (&*name, value) {
            ("secure", None) => cookie.set_secure(true),
            ("httponly", None) => cookie.set_http_only(true),
            ("partitioned", None) => cookie.set_partitioned(true),
            ("max-age", Some(value)) => cookie.max_age = parse_max_age(value, &options),
            ("domain", Some(value)) => cookie.domain = parse_domain(value, parts.ptr, &options),
            ("path", Some(value)) => cookie.path = parse_path(value, parts.ptr, &options),
            ("expires", Some(value)) => cookie.expires = expires::parse_expires(&value),
            ("", Some(_)) if is_strict => {
                // invalid attributes return an error (foo=bar; =10)
                return Err(Error::InvalidAttribute);
            }
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

// set-cookie-header = "Set-Cookie:" SP set-cookie-string
// set-cookie-string = cookie-pair *( ";" SP cookie-av )
// cookie-pair       = cookie-name "=" cookie-value
// cookie-name       = token
// cookie-value      = *cookie-octet / ( DQUOTE *cookie-octet DQUOTE )
// cookie-octet      = %x21 / %x23-2B / %x2D-3A / %x3C-5B / %x5D-7E
//                       ; US-ASCII characters excluding CTLs,
//                       ; whitespace DQUOTE, comma, semicolon,
//                       ; and backslash
// token             = <token, defined in [RFC2616], Section 2.2>

// cookie-av         = expires-av / max-age-av / domain-av /
//                     path-av / secure-av / httponly-av /
//                     extension-av
// expires-av        = "Expires=" sane-cookie-date
// sane-cookie-date  = <rfc1123-date, defined in [RFC2616], Section 3.3.1>
// max-age-av        = "Max-Age=" non-zero-digit *DIGIT
//                       ; In practice, both expires-av and max-age-av
//                       ; are limited to dates representable by the
//                       ; user agent.
// non-zero-digit    = %x31-39
//                       ; digits 1 through 9
// domain-av         = "Domain=" domain-value
// domain-value      = <subdomain>
//                       ; defined in [RFC1034], Section 3.5, as
//                       ; enhanced by [RFC1123], Section 2.1
// path-av           = "Path=" path-value
// path-value        = <any CHAR except CTLs or ";">
// secure-av         = "Secure"
// httponly-av       = "HttpOnly"
// extension-av      = <any CHAR except CTLs or ";">
#[inline]
fn allowed_cookie_value(val: char) -> bool {
    match val {
        ' ' | '"' | ',' | ';' | '\\' => false,
        control if control.is_ascii_control() => false,
        _ => true,
    }
}

#[inline]
pub fn invalid_cookie_value(cookie: &str) -> bool {
    !cookie.chars().all(allowed_cookie_value)
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
