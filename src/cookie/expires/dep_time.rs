use time::{
    OffsetDateTime, PrimitiveDateTime, UtcOffset, format_description::FormatItem,
    macros::format_description, parsing::Parsable,
};

use crate::Cookie;

use super::{Expires, ExpiresInner};

static FMT1: &[FormatItem<'_>] = format_description!(
    "[weekday repr:short], [day] [month repr:short] [year padding:none] [hour]:[minute]:[second] GMT"
);
static FMT2: &[FormatItem<'_>] = format_description!(
    "[weekday], [day]-[month repr:short]-[year repr:last_two] [hour]:[minute]:[second] GMT"
);
static FMT3: &[FormatItem<'_>] = format_description!(
    "[weekday repr:short] [month repr:short] [day padding:space] [hour]:[minute]:[second] [year padding:none]"
);
static FMT4: &[FormatItem<'_>] = format_description!(
    "[weekday repr:short], [day]-[month repr:short]-[year padding:none] [hour]:[minute]:[second] GMT"
);

impl From<OffsetDateTime> for Expires {
    fn from(value: OffsetDateTime) -> Self {
        Self(ExpiresInner::Expires {
            time: Some(value),
            #[cfg(feature = "chrono")]
            chrono: None,
            #[cfg(feature = "jiff")]
            jiff: None,
        })
    }
}

impl Cookie {
    pub fn expires_time(&self) -> Option<&OffsetDateTime> {
        match &self.expires {
            Some(Expires(ExpiresInner::Expires { time, .. })) => time.as_ref(),
            _ => None,
        }
    }

    pub(crate) fn serialize_expires_time(&self, buf: &mut String) -> crate::Result<bool> {
        let Some(expires) = self.expires_time() else {
            return Ok(false);
        };

        let expires = expires
            .to_offset(UtcOffset::UTC)
            .format(&FMT1)
            .map_err(|_| crate::Error::ExpiresFmt)?;

        buf.push_str("; Expires=");
        buf.push_str(&expires);

        Ok(true)
    }
}

pub(crate) fn parse_date(s: &str, format: &impl Parsable) -> Result<OffsetDateTime, time::Error> {
    // Parse. Handle "abbreviated" dates like Chromium. See cookie#162.
    let mut date = format.parse(s.as_bytes())?;
    if let Some(y) = date
        .year()
        .or_else(|| date.year_last_two().map(|v| v as i32))
    {
        let offset = match y {
            0..=68 => 2000,
            69..=99 => 1900,
            _ => 0,
        };

        date.set_year(y + offset);
    }

    Ok(PrimitiveDateTime::try_from(date)?.assume_utc())
}

pub(crate) fn parse_expires_time(value: &str) -> Option<OffsetDateTime> {
    parse_date(value, &FMT1)
        .or_else(|_| parse_date(value, &FMT2))
        .or_else(|_| parse_date(value, &FMT3))
        .or_else(|_| parse_date(value, &FMT4))
        .ok()
}

#[cfg(test)]
mod test_time {
    use crate::Cookie;
    use time::{Date, Month, OffsetDateTime, PrimitiveDateTime, Time};

    #[test]
    fn parse() {
        let expires = offset_datetime(21, 10, 2015, 7, 28, 0);
        let expected = Cookie::build("foo", "bar").expires(expires).build();

        assert_eq!(
            expected.serialize_strict().as_deref(),
            Ok("foo=bar; Expires=Wed, 21 Oct 2015 07:28:00 GMT")
        )
    }

    fn offset_datetime(
        day: u32,
        month: u32,
        year: i32,
        hour: u32,
        min: u32,
        sec: u32,
    ) -> OffsetDateTime {
        let month = Month::try_from(month as u8).unwrap();
        let date = Date::from_calendar_date(year, month, day as u8).unwrap();
        let time = Time::from_hms(hour as u8, min as u8, sec as u8).unwrap();
        PrimitiveDateTime::new(date, time).assume_utc()
    }

    #[test]
    fn parse_abbreviated_years() {
        use crate::cookie::expires::test_cases::ABBREVIATED_YEARS;

        for (cookie, day, month, year, hour, min, sec) in ABBREVIATED_YEARS.to_owned() {
            let expected = offset_datetime(day, month, year, hour, min, sec);

            let found = Cookie::parse_strict(cookie).unwrap();
            let expires = found.expires_time().unwrap();

            assert_eq!(*expires, expected);
        }
    }

    #[test]
    fn parse_variant_date_fmts() {
        use crate::cookie::expires::test_cases::ALTERNATIVE_FMTS;

        for (cookie, day, month, year, hour, min, sec) in ALTERNATIVE_FMTS.to_owned() {
            let expected = offset_datetime(day, month, year, hour, min, sec);

            let found = Cookie::parse_strict(cookie).unwrap();
            let expires = found.expires_time().unwrap();

            assert_eq!(*expires, expected);
        }
    }
}
