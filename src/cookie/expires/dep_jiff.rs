use std::{fmt::Write, sync::LazyLock};

use jiff::{SignedDuration, Span, Zoned, tz::TimeZone};

use crate::{Cookie, Error, cookie::expires::ExpVal};

use super::Expires;

// Sun, 06 Nov 1994 08:49:37 GMT (RFC)
static FMT: &str = "%a, %d %b %Y %T %Z";

impl From<Zoned> for Expires {
    fn from(value: Zoned) -> Self {
        Self::Exp(super::ExpVal {
            jiff: Some(value),
            ..Default::default()
        })
    }
}

impl Cookie {
    pub fn expires_jiff(&self) -> Option<&Zoned> {
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

pub(super) fn ser_expires(expires: &Zoned, buf: &mut String) -> crate::Result<()> {
    static GMT: LazyLock<TimeZone> =
        LazyLock::new(|| TimeZone::get("GMT").expect("Failed to look up `GMT` timezone"));

    let zoned = expires.with_time_zone(GMT.clone());
    write!(buf, "; Expires={}", zoned.strftime(FMT)).map_err(|_| Error::ExpiresFmt)
}

#[cfg(test)]
mod test_jiff {
    use crate::Cookie;
    use jiff::{Zoned, civil::datetime};

    #[test]
    fn zoned() {
        let now = Zoned::now();

        let _ = Cookie::build("key", "value").expires(now);
    }

    #[test]
    fn parse() {
        let expires = datetime(2015, 10, 21, 7, 28, 0, 0).in_tz("GMT").unwrap();

        let expected = Cookie::build("foo", "bar").expires(expires).build();

        assert_eq!(
            expected.serialize().as_deref(),
            Ok("foo=bar; Expires=Wed, 21 Oct 2015 07:28:00 GMT")
        )
    }
}
