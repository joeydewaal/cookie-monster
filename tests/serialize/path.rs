use cookie_monster::{Cookie, Error};

use crate::assert_eq_ser;

#[test]
fn path() {
    assert_eq_ser!(
        Cookie::build("foo", "bar").path("/home").build(),
        Ok("foo=bar; Path=/home")
    );

    assert_eq_ser!(
        Cookie::build("foo", "bar").path("home").build(),
        Err(&Error::InvalidPathValue)
    );
    assert_eq_ser!(
        Cookie::build("foo", "bar").path("").build(),
        Err(&Error::EmptyPathValue)
    );
}
