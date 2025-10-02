#[cfg(all(feature = "jiff", feature = "chrono", feature = "time"))]
mod time_jiff_chrono {
    use chrono::{DateTime, Utc};
    use cookie_monster::Cookie;

    #[test]
    // #[should_panic]
    fn all() {
        let _ = Cookie::build("foo", "bar")
            .expires(jiff::Timestamp::now())
            .expires(DateTime::<Utc>::MAX_UTC)
            .expires(time::OffsetDateTime::now_utc())
            .build()
            .serialize();
    }
}
