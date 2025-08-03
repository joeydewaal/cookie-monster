use std::time::Duration;

use cookie_monster::Cookie;

mod util;

#[test]
fn max_age() {
    assert_eq_ser!(
        Cookie::build("foo", "bar").max_age(Duration::ZERO).build(),
        all = "foo=bar; Max-Age=0"
    );
    assert_eq_ser!(
        Cookie::build("foo", "bar")
            .max_age(Duration::from_secs(12))
            .build(),
        all = "foo=bar; Max-Age=12"
    );
}
