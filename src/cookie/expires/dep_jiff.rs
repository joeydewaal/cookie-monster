use std::fmt::Write;

use jiff::{SignedDuration, Span, Timestamp, Zoned};

use crate::{Cookie, Error, cookie::expires::ExpVal};

use super::{Expires, formats::FMT1};

impl From<Timestamp> for Expires {
    fn from(value: Timestamp) -> Self {
        Self::Exp(super::ExpVal {
            jiff: Some(value),
            ..Default::default()
        })
    }
}

impl From<Zoned> for Expires {
    fn from(value: Zoned) -> Self {
        Self::Exp(super::ExpVal {
            jiff: Some(value.timestamp()),
            ..Default::default()
        })
    }
}

impl Cookie {
    pub fn expires_jiff(&self) -> Option<&Timestamp> {
        match &self.expires {
            Expires::Exp(ExpVal { jiff, .. }) => jiff.as_ref(),
            _ => None,
        }
    }

    pub fn max_age_jiff(&self) -> Option<SignedDuration> {
        self.max_age_secs()
            .map(|max_age| SignedDuration::from_secs(max_age as i64))
    }
}

impl Expires {
    pub(crate) fn remove_jiff() -> Self {
        Self::from(&Zoned::now() - Span::new().years(1))
    }
}

pub(super) fn ser_expires(expires: Timestamp, buf: &mut String) -> crate::Result<()> {
    write!(buf, "; Expires={}", expires.strftime(FMT1)).map_err(|_| Error::ExpiresFmt)
}

#[cfg(test)]
mod test_jiff {
    use crate::Cookie;
    use jiff::{Timestamp, Zoned, civil::datetime, tz::TimeZone};

    #[test]
    fn zoned() {
        let now = Zoned::now();

        let _ = Cookie::build("key", "value").expires(now);
    }

    #[test]
    fn parse() {
        let expires = timestamp(21, 10, 2015, 7, 28, 0);

        let expected = Cookie::build("foo", "bar").expires(expires).build();

        assert_eq!(
            expected.serialize().as_deref(),
            Ok("foo=bar; Expires=Wed, 21 Oct 2015 07:28:00 GMT")
        )
    }

    fn timestamp(day: u32, month: u32, year: i32, hour: u32, min: u32, sec: u32) -> Timestamp {
        datetime(
            year as _, month as _, day as _, hour as _, min as _, sec as _, 0,
        )
        .to_zoned(TimeZone::UTC)
        .unwrap()
        .timestamp()
    }
}
