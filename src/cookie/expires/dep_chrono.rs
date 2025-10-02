use chrono::{
    DateTime, NaiveDate, NaiveDateTime, NaiveTime, Utc,
    format::{Parsed, StrftimeItems},
};

use crate::{Cookie, Error};

use super::{
    Expires, Inner,
    formats::{FMT1, FMT2, FMT3, FMT4},
};

static PARTS: [StrftimeItems<'static>; 4] = [
    StrftimeItems::new(FMT1),
    StrftimeItems::new(FMT2),
    StrftimeItems::new(FMT3),
    StrftimeItems::new(FMT4),
];

static MAX_EXPIRES: DateTime<Utc> = NaiveDateTime::new(
    NaiveDate::from_ymd_opt(9999, 12, 31).unwrap(),
    NaiveTime::from_hms_micro_opt(23, 59, 59, 59).unwrap(),
)
.and_utc();

impl From<DateTime<Utc>> for Expires {
    fn from(value: DateTime<Utc>) -> Self {
        Self(Inner::Exp {
            #[cfg(feature = "time")]
            time: None,
            chrono: Some(std::cmp::min(value, MAX_EXPIRES)),
            #[cfg(feature = "jiff")]
            jiff: None,
        })
    }
}

impl Cookie {
    pub fn expires_chrono(&self) -> Option<&DateTime<Utc>> {
        match &self.expires {
            Expires(Inner::Exp { chrono, .. }) => chrono.as_ref(),
            _ => None,
        }
    }
}
pub(super) fn ser_expires(expires: DateTime<Utc>, buf: &mut String) -> crate::Result<()> {
    buf.push_str("; Expires=");

    expires
        .format(FMT1)
        .write_to(buf)
        .map_err(|_| Error::ExpiresFmt)
}

pub fn parse_expires(value: &str) -> Option<DateTime<Utc>> {
    for format in PARTS.clone() {
        let mut parsed = Parsed::new();

        if chrono::format::parse(&mut parsed, value, format).is_ok() {
            if let Some(year) = parsed.year().or_else(|| parsed.year_mod_100()) {
                let offset = match year {
                    0..=68 => 2000,
                    69..=99 => 1900,
                    _ => 0,
                };

                let _ = parsed.set_year(year as i64 + offset).ok();
            }
            let expires = parsed.to_datetime_with_timezone(&chrono::Utc).ok()?;
            return Some(std::cmp::min(expires, MAX_EXPIRES));
        }
    }
    None
}

#[cfg(test)]
mod test_chrono {
    use crate::{Cookie, cookie::expires::dep_chrono::MAX_EXPIRES};
    use chrono::{Duration, TimeZone, Utc};

    #[test]
    fn parse() {
        let expires = Utc.with_ymd_and_hms(2015, 10, 21, 7, 28, 0).unwrap();

        let expected = Cookie::build("foo", "bar").expires(expires).build();

        assert_eq!(
            expected.serialize().as_deref(),
            Ok("foo=bar; Expires=Wed, 21 Oct 2015 07:28:00 GMT")
        )
    }

    #[test]
    fn parse_abbreviated_years() {
        use crate::cookie::expires::test_cases::ABBREVIATED_YEARS;

        for (cookie, day, month, year, hour, min, sec) in ABBREVIATED_YEARS.to_owned() {
            let expected = Utc
                .with_ymd_and_hms(year, month, day, hour, min, sec)
                .unwrap();

            let found = Cookie::parse(cookie).unwrap();
            let expires = found.expires_chrono().unwrap();

            assert_eq!(expires, &expected);
        }
    }

    #[test]
    fn parse_variant_date_fmts() {
        use crate::cookie::expires::test_cases::ALTERNATIVE_FMTS;

        for (cookie, day, month, year, hour, min, sec) in ALTERNATIVE_FMTS.to_owned() {
            let expected = Utc
                .with_ymd_and_hms(year, month, day, hour, min, sec)
                .unwrap();

            let found = Cookie::parse(cookie).unwrap();
            let expires = found.expires_chrono().unwrap();

            assert_eq!(expires, &expected);
        }
    }

    #[test]
    fn large_date() {
        assert_eq!(
            Cookie::build("foo", "bar")
                .expires(MAX_EXPIRES + Duration::weeks(1))
                .build()
                .serialize()
                .as_deref(),
            Ok("foo=bar; Expires=Fri, 31 Dec 9999 23:59:59 GMT")
        );

        let cookie = Cookie::parse("foo=bar; Expires=Fri, 07 Jan +10000 23:59:59 GMT").unwrap();
        assert_eq!(cookie.expires_chrono(), Some(&MAX_EXPIRES));
    }
}
