use cookie_monster::Cookie;

use crate::assert_eq_ser;

#[test]
fn domain() {
    assert_eq_ser!(
        Cookie::build("foo", "bar").domain("rust-lang.com").build(),
        Ok("foo=bar; Domain=rust-lang.com")
    );

    assert_eq_ser!(
        Cookie::build("foo", "bar").domain(".rust-lang.com").build(),
        Ok("foo=bar; Domain=rust-lang.com")
    );

    assert_eq_ser!(
        Cookie::build("foo", "bar").domain("").build(),
        Ok("foo=bar")
    );

    assert_eq_ser!(
        Cookie::build("foo", "bar")
            .domain("rust-lang\0.com")
            .build(),
        Ok("foo=bar")
    );
}
