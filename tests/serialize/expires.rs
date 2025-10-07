#[cfg(all(feature = "jiff", feature = "chrono", feature = "time"))]
mod time_jiff_chrono {
    use chrono::{DateTime, Utc};
    use cookie_monster::Cookie;
    use jiff::{civil::datetime, tz::TimeZone};

    use crate::assert_eq_ser;

    #[test]
    fn all() {
        // Only last one set takes effect.
        let expires = datetime(2025, 10, 10, 23, 37, 0, 0)
            .to_zoned(TimeZone::UTC)
            .unwrap();

        assert_eq_ser!(
            Cookie::build("foo", "bar")
                .expires(DateTime::<Utc>::MAX_UTC)
                .expires(time::OffsetDateTime::now_utc())
                .expires(expires)
                .build(),
            Ok("foo=bar; Expires=Fri, 10 Oct 2025 23:37:00 GMT")
        );
    }
}
