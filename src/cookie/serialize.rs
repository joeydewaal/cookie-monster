use crate::Error;

use super::{Cookie, options::CookieOptions, parse::invalid_cookie_value};
use std::fmt::Write;

impl Cookie {
    pub fn serialize_strict(&self) -> crate::Result<String> {
        self.serialize_inner(CookieOptions::strict())
    }

    pub fn serialize_relaxed(&self) -> crate::Result<String> {
        self.serialize_inner(CookieOptions::relaxed())
    }

    pub fn serialize_unchecked(&self) -> String {
        self.serialize_inner(CookieOptions::unchecked())
            .expect("Unchecked serialize should not return an error")
    }

    fn serialize_inner(&self, opts: CookieOptions) -> crate::Result<String> {
        let is_unchecked = opts.is_unchecked();

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

        self.serialize_domain(&mut buf, &opts)?;

        self.serialize_path(&mut buf, &opts)?;

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

// #[cfg(test)]
// mod serialize_test {
//     use std::time::Duration;

//     use crate::{Cookie, assert_eq_ser};

//     // #[test]
//     // fn ser_cookie() {
//     //     assert_eq_ser!(Cookie::build("foo", "bar"), "foo=bar");
//     //     assert_eq_ser!(Cookie::build("foo", "\"bar\""), "foo=bar");
//     // }

//     #[test]
//     fn ser_duration() {
//         assert_eq_ser!(
//             Cookie::build("foo", "bar").max_age(Duration::ZERO),
//             "foo=bar; Max-Age=0"
//         );
//         assert_eq_ser!(
//             Cookie::build("foo", "bar").max_age(Duration::from_secs(10)),
//             "foo=bar; Max-Age=10"
//         );
//         assert_eq_ser!(
//             Cookie::build("foo", "bar").max_age(Duration::from_millis(1)),
//             "foo=bar; Max-Age=0"
//         );
//     }

//     #[test]
//     fn ser_http_only() {
//         assert_eq_ser!(Cookie::build("foo", "bar").http_only(), "foo=bar; HttpOnly");
//     }

//     #[test]
//     fn ser_partitioned() {
//         assert_eq_ser!(
//             Cookie::build("foo", "bar").partitioned(),
//             "foo=bar; Partitioned"
//         );
//     }

//     #[test]
//     fn ser_secure() {
//         assert_eq_ser!(Cookie::build("foo", "bar").secure(), "foo=bar; Secure");
//     }

//     #[test]
//     fn ser_path() {
//         assert_eq_ser!(
//             Cookie::build("foo", "bar").path("/home"),
//             "foo=bar; Path=/home"
//         );

//         assert_eq_ser!(Cookie::build("foo", "bar").path("home"), "foo=bar");
//         assert_eq_ser!(Cookie::build("foo", "bar").path(""), "foo=bar");
//         assert_eq_ser!(Cookie::build("foo", "bar").path("\0"), "foo=bar");
//     }

//     #[test]
//     fn ser_domain() {
//         assert_eq_ser!(
//             Cookie::build("foo", "bar").domain("www.rust-lang.com"),
//             "foo=bar; Domain=www.rust-lang.com"
//         );

//         // assert_eq_ser!(
//         //     Cookie::build("foo", "bar").domain(".www.rust-lang.com"),
//         //     "foo=bar; Domain=www.rust-lang.com"
//         // );

//         // assert_eq_ser!(Cookie::build("foo", "bar").domain("\0"), "foo=bar");
//     }
// }
