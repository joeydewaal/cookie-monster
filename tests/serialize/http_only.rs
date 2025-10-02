use cookie_monster::Cookie;

use crate::assert_eq_ser;

#[test]
fn http_only() {
    assert_eq_ser!(
        Cookie::build("foo", "bar").http_only().build(),
        Ok("foo=bar; HttpOnly")
    );
    assert_eq_ser!(
        Cookie::build("foo", "").http_only().build(),
        Ok("foo=; HttpOnly")
    );
}
