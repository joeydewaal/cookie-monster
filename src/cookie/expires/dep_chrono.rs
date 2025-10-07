use chrono::{DateTime, Duration, NaiveDate, NaiveDateTime, NaiveTime, Utc};

use crate::{Cookie, Error, cookie::expires::ExpVal};

use super::{Expires, formats::FMT1};

static MAX_EXPIRES: DateTime<Utc> = NaiveDateTime::new(
    NaiveDate::from_ymd_opt(9999, 12, 31).unwrap(),
    NaiveTime::from_hms_micro_opt(23, 59, 59, 59).unwrap(),
)
.and_utc();

impl From<DateTime<Utc>> for Expires {
    fn from(value: DateTime<Utc>) -> Self {
        Self::Exp(ExpVal {
            chrono: Some(std::cmp::min(value, MAX_EXPIRES)),
            ..Default::default()
        })
    }
}

impl Cookie {
    pub fn expires_chrono(&self) -> Option<&DateTime<Utc>> {
        match &self.expires {
            Expires::Exp(ExpVal { chrono, .. }) => chrono.as_ref(),
            _ => None,
        }
    }

    pub fn max_age_chrono(&self) -> Option<Duration> {
        self.max_age_secs()
            .map(|max_age| Duration::seconds(max_age as i64))
    }
}

impl Expires {
    pub(crate) fn remove_chrono() -> Self {
        Self::from(Utc::now() - Duration::days(365))
    }
}

pub(super) fn ser_expires(expires: DateTime<Utc>, buf: &mut String) -> crate::Result<()> {
    buf.push_str("; Expires=");

    expires
        .format(FMT1)
        .write_to(buf)
        .map_err(|_| Error::ExpiresFmt)
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
    fn large_date() {
        assert_eq!(
            Cookie::build("foo", "bar")
                .expires(MAX_EXPIRES + Duration::weeks(1))
                .build()
                .serialize()
                .as_deref(),
            Ok("foo=bar; Expires=Fri, 31 Dec 9999 23:59:59 GMT")
        );
    }
}
