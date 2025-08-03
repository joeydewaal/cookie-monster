use chrono::{
    DateTime, Utc,
    format::{Parsed, StrftimeItems},
};

use crate::{Cookie, Error};

use super::{
    Expires, ExpiresInner,
    formats::{FMT1, FMT2, FMT3, FMT4},
};

static PARTS: [StrftimeItems<'static>; 4] = [
    StrftimeItems::new(FMT1),
    StrftimeItems::new(FMT2),
    StrftimeItems::new(FMT3),
    StrftimeItems::new(FMT4),
];

impl From<DateTime<Utc>> for Expires {
    fn from(value: DateTime<Utc>) -> Self {
        Self(ExpiresInner::Expires {
            #[cfg(feature = "time")]
            time: None,
            chrono: Some(value),
            #[cfg(feature = "jiff")]
            jiff: None,
        })
    }
}

impl Cookie {
    pub fn expires_chrono(&self) -> Option<&DateTime<Utc>> {
        match &self.expires {
            Some(Expires(ExpiresInner::Expires { chrono, .. })) => chrono.as_ref(),
            _ => None,
        }
    }

    pub(crate) fn serialize_expires_chrono(&self, buf: &mut String) -> crate::Result<bool> {
        let Some(expires) = self.expires_chrono() else {
            return Ok(false);
        };

        let utc_expires = expires.to_utc();

        buf.push_str("; Expires=");
        utc_expires
            .format(FMT1)
            .write_to(buf)
            .map_err(|_| Error::ExpiresFmt)?;
        Ok(true)
    }
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
            return parsed.to_datetime_with_timezone(&chrono::Utc).ok();
        }
    }
    None
}

#[cfg(test)]
mod test_chrono {
    use crate::Cookie;
    use chrono::{TimeZone, Utc};

    #[test]
    fn parse() {
        let expires = Utc.with_ymd_and_hms(2015, 10, 21, 7, 28, 0).unwrap();

        let expected = Cookie::build("foo", "bar").expires(expires).build();

        assert_eq!(
            expected.serialize_strict().as_deref(),
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

            let found = Cookie::parse_strict(cookie).unwrap();
            let expires = found.expires_chrono().unwrap();

            assert_eq!(*expires, expected);
        }
    }

    #[test]
    fn parse_variant_date_fmts() {
        use crate::cookie::expires::test_cases::ALTERNATIVE_FMTS;

        for (cookie, day, month, year, hour, min, sec) in ALTERNATIVE_FMTS.to_owned() {
            let expected = Utc
                .with_ymd_and_hms(year, month, day, hour, min, sec)
                .unwrap();

            let found = Cookie::parse_strict(cookie).unwrap();
            let expires = found.expires_chrono().unwrap();

            assert_eq!(*expires, expected);
        }
    }
}
