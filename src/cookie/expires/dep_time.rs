use time::{
    OffsetDateTime, PrimitiveDateTime, UtcOffset, format_description::FormatItem,
    macros::format_description, parsing::Parsable,
};

use crate::{Cookie, cookie::expires::ExpVal};

use super::Expires;

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
        // The expires hear cannot be above 9999 years but this is also the upper bound of
        // `time::OffsetDateTime`.
        Self::Exp(ExpVal {
            time: Some(value),
            ..Default::default()
        })
    }
}

impl Cookie {
    pub fn expires_time(&self) -> Option<&OffsetDateTime> {
        match &self.expires {
            Expires::Exp(ExpVal { time, .. }) => time.as_ref(),
            _ => None,
        }
    }
}
pub(super) fn ser_expires(expires: OffsetDateTime, buf: &mut String) -> crate::Result<()> {
    let expires = expires
        .to_offset(UtcOffset::UTC)
        .format(&FMT1)
        .map_err(|_| crate::Error::ExpiresFmt)?;

    buf.push_str("; Expires=");
    buf.push_str(&expires);

    Ok(())
}

pub(crate) fn parse_date(s: &str, format: &impl Parsable) -> Result<OffsetDateTime, time::Error> {
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
            expected.serialize().as_deref(),
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

            let found = Cookie::parse_set_cookie(cookie).unwrap();
            let expires = found.expires_time().unwrap();

            assert_eq!(expires, &expected);
        }
    }

    #[test]
    fn parse_variant_date_fmts() {
        use crate::cookie::expires::test_cases::ALTERNATIVE_FMTS;

        for (cookie, day, month, year, hour, min, sec) in ALTERNATIVE_FMTS.to_owned() {
            let expected = offset_datetime(day, month, year, hour, min, sec);

            let found = Cookie::parse_set_cookie(cookie).unwrap();
            let expires = found.expires_time().unwrap();

            assert_eq!(expires, &expected);
        }
    }
}
