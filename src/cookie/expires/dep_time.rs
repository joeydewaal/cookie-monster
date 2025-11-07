use time::{
    Duration, OffsetDateTime, UtcOffset, format_description::FormatItem, macros::format_description,
};

use crate::{Cookie, cookie::expires::ExpVal};

use super::Expires;

static FMT: &[FormatItem<'_>] = format_description!(
    "[weekday repr:short], [day] [month repr:short] [year padding:none] [hour]:[minute]:[second] GMT"
);

impl From<OffsetDateTime> for Expires {
    fn from(value: OffsetDateTime) -> Self {
        Self::Exp(ExpVal {
            time: Some(value),
            ..Default::default()
        })
    }
}

impl Cookie {
    /// Returns the Expires attribute using a [`OffsetDateTime`](time::OffsetDateTime).
    pub fn expires_time(&self) -> Option<OffsetDateTime> {
        match &self.expires {
            Expires::Exp(ExpVal { time, .. }) => *time,
            _ => None,
        }
    }

    /// Returns the Max-Age attribute using a [`Duration`](time::Duration).
    pub fn max_age_time(&self) -> Option<Duration> {
        self.max_age_secs()
            .map(|max_age| Duration::seconds(max_age as i64))
    }
}

impl Expires {
    /// Creates an [`Expires`] with a [`OffsetDateTime`](time::OffsetDateTime) one year in the past.
    pub fn remove_time() -> Self {
        Self::from(OffsetDateTime::now_utc() - Duration::days(365))
    }
}

pub(super) fn ser_expires(expires: &OffsetDateTime, buf: &mut String) -> crate::Result<()> {
    let expires = expires
        .to_offset(UtcOffset::UTC)
        .format(&FMT)
        .map_err(|_| crate::Error::ExpiresFmt)?;

    buf.push_str("; Expires=");
    buf.push_str(&expires);

    Ok(())
}

#[cfg(test)]
mod test_time {
    use crate::Cookie;
    use time::macros::datetime;

    #[test]
    fn parse() {
        let expires = datetime!(2015-10-21 7:28:0 UTC);

        let expected = Cookie::build("foo", "bar").expires(expires).build();

        assert_eq!(
            expected.serialize().as_deref(),
            Ok("foo=bar; Expires=Wed, 21 Oct 2015 07:28:00 GMT")
        )
    }
}
