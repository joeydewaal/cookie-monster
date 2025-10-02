use std::time::Duration;

use cookie_monster::Cookie;

use crate::assert_eq_ser;

#[test]
fn max_age() {
    assert_eq_ser!(
        Cookie::build("foo", "bar").max_age(Duration::ZERO).build(),
        Ok("foo=bar; Max-Age=0")
    );
    assert_eq_ser!(
        Cookie::build("foo", "bar")
            .max_age(Duration::from_secs(12))
            .build(),
        Ok("foo=bar; Max-Age=12")
    );
}
