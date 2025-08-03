use cookie_monster::{Cookie, Error};

mod util;

#[test]
fn path() {
    assert_eq_ser!(
        Cookie::build("foo", "bar").path("/home").build(),
        all = "foo=bar; Path=/home"
    );

    assert_eq_ser!(
        Cookie::build("foo", "bar").path("").build(),
        strict = Err(&Error::InvalidAttribute),
        relaxed = Ok("foo=bar"),
        unchecked = "foo=bar; Path="
    );

    assert_eq_ser!(
        Cookie::build("foo", "bar").path("home").build(),
        strict = Err(&Error::InvalidAttribute),
        unchecked = "foo=bar; Path=home"
    );
}
