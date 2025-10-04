use cookie_monster::Cookie;

use crate::assert_eq_ser_enc;

#[test]
fn encoded() {
    assert_eq_ser_enc!(Cookie::new("foo", "bar\0"), Ok("foo=bar%00"));

    assert_eq_ser_enc!(
        Cookie::new("foo !%?=", "bar;;, a"),
        Ok("foo%20!%%3F%3D=bar%3B%3B%2C%20a")
    );
}
