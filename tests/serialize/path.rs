use cookie_monster::{Cookie, Error};

mod util;

#[test]
fn path() {
    assert_eq_ser!(
        Cookie::build("foo", "bar").path("/home").build(),
        Ok("foo=bar; Path=/home")
    );

    assert_eq_ser!(Cookie::build("foo", "bar").path("").build(), Ok("foo=bar"));

    assert_eq_ser!(
        Cookie::build("foo", "bar").path("home").build(),
        Err(&Error::InvalidAttribute)
    );
}
