use std::fmt::Write;

use jiff::{Timestamp, fmt::strtime::BrokenDownTime, tz::Offset};

use crate::{Cookie, CookieBuilder, Error};

use super::{
    Expires, ExpiresInner,
    formats::{FMT1, FMT2, FMT3, FMT4, FMT5},
};

impl From<Timestamp> for Expires {
    fn from(value: Timestamp) -> Self {
        Self(ExpiresInner::Expires {
            #[cfg(feature = "time")]
            time: None,
            #[cfg(feature = "chrono")]
            chrono: None,
            #[cfg(feature = "jiff")]
            jiff: Some(value),
        })
    }
}

impl Cookie {
    pub fn expires_jiff(&self) -> Option<&Timestamp> {
        match &self.expires {
            Some(Expires(ExpiresInner::Expires { jiff, .. })) => jiff.as_ref(),
            _ => None,
        }
    }

    pub(crate) fn serialize_expires_jiff(&self, buf: &mut String) -> crate::Result<()> {
        let Some(expires) = self.expires_jiff() else {
            return Ok(());
        };

        write!(buf, "; Expires={}", expires.strftime(FMT1)).map_err(|_| Error::ExpiresFmt)?;
        Ok(())
    }
}

impl CookieBuilder {}

pub fn parse_expires(attribute: &str) -> Option<Timestamp> {
    let mut parsed = BrokenDownTime::parse(FMT1, attribute)
        .or_else(|_| BrokenDownTime::parse(FMT2, attribute))
        .or_else(|_| BrokenDownTime::parse(FMT3, attribute))
        .or_else(|_| BrokenDownTime::parse(FMT4, attribute))
        .or_else(|_| BrokenDownTime::parse(FMT5, attribute))
        .ok()?;

    if let Some(year) = parsed.year() {
        let offset = match year {
            0..=68 => 2000,
            69..=99 => 1900,
            _ => 0,
        };
        parsed.set_year(Some(year + offset)).ok()?;
    }

    parsed.set_offset(Some(Offset::UTC));
    parsed.to_timestamp().ok()
}

#[cfg(test)]
mod test_jiff {
    use crate::Cookie;
    use jiff::{Timestamp, civil::datetime, tz::TimeZone};

    #[test]
    fn parse() {
        let expires = timestamp(21, 10, 2015, 7, 28, 0);

        let expected = Cookie::build("foo", "bar").expires(expires).build();

        assert_eq!(
            expected.serialize_strict().as_deref(),
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

    #[test]
    fn parse_abbreviated_years() {
        use crate::cookie::expires::test_cases::ABBREVIATED_YEARS;

        for (cookie, day, month, year, hour, min, sec) in ABBREVIATED_YEARS.to_owned() {
            let expected = timestamp(day, month, year, hour, min, sec);

            let found = Cookie::parse_strict(cookie).unwrap();
            let expires = found.expires_jiff().unwrap();

            assert_eq!(*expires, expected);
        }
    }

    #[test]
    fn parse_variant_date_fmts() {
        use crate::cookie::expires::test_cases::ALTERNATIVE_FMTS;

        for (cookie, day, month, year, hour, min, sec) in ALTERNATIVE_FMTS.to_owned() {
            let expected = timestamp(day, month, year, hour, min, sec);

            let found = Cookie::parse_strict(cookie).unwrap();
            let expires = found.expires_jiff().unwrap();

            assert_eq!(*expires, expected);
        }
    }
}
