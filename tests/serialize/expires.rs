// mod util;

#[cfg(all(feature = "jiff", feature = "chrono", feature = "time"))]
mod time_jiff_chrono {
    use chrono::{DateTime, Utc};
    use cookie_monster::Cookie;

    use crate::assert_eq_ser;

    #[test]
    #[ignore = "reason"]
    fn all() {
        let cookie = Cookie::build("foo", "bar")
            .expires(jiff::Timestamp::now())
            .expires(DateTime::<Utc>::MAX_UTC)
            .expires(time::OffsetDateTime::now_utc())
            .build();

        assert_eq_ser!(cookie, Ok("foo=bar"));
    }
}
