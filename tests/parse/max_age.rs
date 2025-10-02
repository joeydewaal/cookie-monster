use std::time::Duration;

use cookie_monster::Cookie;

use crate::{assert_eq_parse, assert_ne_parse};

#[test]
fn max_age() {
    let cookie = || Ok(Cookie::build("foo", "bar").max_age(Duration::ZERO).build());
    assert_eq_parse!("foo=bar; Max-Age=0", cookie());
    assert_eq_parse!("foo=bar; max-age=0", cookie());
    assert_eq_parse!("foo=bar; MAX-AGE=0", cookie());
    assert_eq_parse!("foo=bar; Max-Age=-0", cookie());
    assert_eq_parse!("foo=bar; Max-Age=-10", cookie());

    let cookie = || {
        Ok(Cookie::build("foo", "bar")
            .max_age(Duration::from_secs(10))
            .build())
    };

    assert_eq_parse!("foo=bar; Max-Age=10", cookie());
    assert_eq_parse!("foo=bar; max-age= 10", cookie());
    assert_eq_parse!("foo=bar; MAX-AGE=  10", cookie());
    assert_eq_parse!("foo=bar; Max-Age=10 ", cookie());
    assert_eq_parse!("foo=bar; Max-Age=10  ", cookie());
    assert_eq_parse!("foo=bar; Max-Age=  10  ", cookie());
}

#[test]
fn invalid_max_age() {
    let cookie = || {
        Ok(Cookie::build("foo", "bar")
            .max_age(Duration::from_secs(10))
            .build())
    };

    assert_eq_parse!("foo=bar; Max-Age=abc", Ok(Cookie::new("foo", "bar")));
    assert_eq_parse!("foo=bar; Max-Age=+10", cookie());
    assert_eq_parse!("foo=bar; Max-Age=-+1", Ok(Cookie::new("foo", "bar")));
    assert_eq_parse!("foo=bar; Max-Age=", Ok(Cookie::new("foo", "bar")));

    let cookie = || {
        Ok(Cookie::build("foo", "bar")
            .max_age(Duration::from_secs(u64::max_value()))
            .build())
    };

    assert_ne_parse!(
        format!("foo=bar; Max-Age={}", (u64::max_value() as u128) + 1),
        cookie()
    );
}
