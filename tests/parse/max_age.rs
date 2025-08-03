use std::time::Duration;

use cookie_monster::Cookie;

mod util;

#[test]
fn max_age() {
    let cookie = || Cookie::build("foo", "bar").max_age(Duration::ZERO).build();
    assert_eq_parse!("foo=bar; Max-Age=0", all = cookie());
    assert_eq_parse!("foo=bar; max-age=0", all = cookie());
    assert_eq_parse!("foo=bar; MAX-AGE=0", all = cookie());
    assert_eq_parse!("foo=bar; Max-Age=-0", all = cookie());
    assert_eq_parse!("foo=bar; Max-Age=-10", all = cookie());

    let cookie = || {
        Cookie::build("foo", "bar")
            .max_age(Duration::from_secs(10))
            .build()
    };

    assert_eq_parse!("foo=bar; Max-Age=10", all = cookie());
    assert_eq_parse!("foo=bar; max-age= 10", all = cookie());
    assert_eq_parse!("foo=bar; MAX-AGE=  10", all = cookie());
    assert_eq_parse!("foo=bar; Max-Age=10 ", all = cookie());
    assert_eq_parse!("foo=bar; Max-Age=10  ", all = cookie());
    assert_eq_parse!("foo=bar; Max-Age=  10  ", all = cookie());
}

#[test]
fn invalid_max_age() {
    let cookie = || {
        Cookie::build("foo", "bar")
            .max_age(Duration::from_secs(10))
            .build()
    };

    assert_eq_parse!("foo=bar; Max-Age=abc", all = Cookie::new("foo", "bar"));
    assert_eq_parse!(
        "foo=bar; Max-Age=+10",
        parse = Ok(cookie()),
        unchecked = cookie()
    );
    assert_eq_parse!("foo=bar; Max-Age=-+1", all = Cookie::new("foo", "bar"));
    assert_eq_parse!("foo=bar; Max-Age=", all = Cookie::new("foo", "bar"));

    let cookie = || {
        Cookie::build("foo", "bar")
            .max_age(Duration::from_secs(u64::max_value()))
            .build()
    };

    assert_ne_parse!(
        format!("foo=bar; Max-Age={}", (u64::max_value() as u128) + 1),
        all = cookie()
    );
}
