use time::{
    Duration, OffsetDateTime, UtcOffset, format_description::FormatItem, macros::format_description,
};

use crate::{Cookie, cookie::expires::ExpVal};

use super::Expires;

static FMT1: &[FormatItem<'_>] = format_description!(
    "[weekday repr:short], [day] [month repr:short] [year padding:none] [hour]:[minute]:[second] GMT"
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

    pub fn max_age_time(&self) -> Option<Duration> {
        self.max_age_secs()
            .map(|max_age| Duration::seconds(max_age as i64))
    }
}

impl Expires {
    pub(crate) fn remove_time() -> Self {
        Self::from(OffsetDateTime::now_utc() - Duration::days(365))
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
}
