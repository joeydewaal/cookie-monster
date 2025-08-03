use cookie_monster::{Cookie, Error};

mod util;

#[test]
fn domain() {
    assert_eq_ser!(
        Cookie::build("foo", "bar").domain("rust-lang.com").build(),
        all = "foo=bar; Domain=rust-lang.com"
    );

    assert_eq_ser!(
        Cookie::build("foo", "bar").domain(".rust-lang.com").build(),
        strict = Ok("foo=bar; Domain=rust-lang.com"),
        unchecked = "foo=bar; Domain=.rust-lang.com"
    );

    assert_eq_ser!(
        Cookie::build("foo", "bar").domain("").build(),
        strict = Ok("foo=bar"),
        unchecked = "foo=bar; Domain="
    );

    assert_eq_ser!(
        Cookie::build("foo", "bar")
            .domain("rust-lang\0.com")
            .build(),
        strict = Err(&Error::InvalidAttribute),
        relaxed = Ok("foo=bar"),
        unchecked = "foo=bar; Domain=rust-lang\0.com"
    );
}
