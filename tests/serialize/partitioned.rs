use cookie_monster::Cookie;

use crate::assert_eq_ser;

#[test]
fn partitioned() {
    assert_eq_ser!(
        Cookie::build("foo", "bar").partitioned().build(),
        Ok("foo=bar; Secure; Partitioned")
    );
    assert_eq_ser!(
        Cookie::build("foo", "").partitioned().build(),
        Ok("foo=; Secure; Partitioned")
    );
}
