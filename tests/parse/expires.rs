// Ensure that Expires still works with all features enabled.

#[cfg(all(feature = "jiff", feature = "chrono", feature = "time"))]
mod time_jiff_chrono {
    use chrono::{TimeZone as _, Utc};
    use cookie_monster::Cookie;
    use jiff::{civil::datetime, tz::TimeZone};
    use time::macros::datetime;

    use crate::assert_eq_parse;

    #[test]
    fn jiff() {
        let expires = datetime(2025, 10, 10, 23, 37, 0, 0)
            .to_zoned(TimeZone::UTC)
            .unwrap()
            .timestamp();

        assert_eq_parse!(
            "foo=bar; Expires=Fri, 10 Oct 2025 23:37:00 GMT",
            Ok(Cookie::build("foo", "bar").expires(expires).build())
        );
    }

    #[test]
    fn chrono() {
        let expires = Utc.with_ymd_and_hms(2025, 10, 10, 23, 37, 0).unwrap();

        assert_eq_parse!(
            "foo=bar; Expires=Fri, 10 Oct 2025 23:37:00 GMT",
            Ok(Cookie::build("foo", "bar").expires(expires).build())
        );
    }

    #[test]
    fn time() {
        let expires = datetime!(2025-10-10 23:37:00 UTC);

        assert_eq_parse!(
            "foo=bar; Expires=Fri, 10 Oct 2025 23:37:00 GMT",
            Ok(Cookie::build("foo", "bar").expires(expires).build())
        );
    }
}
