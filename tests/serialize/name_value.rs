use cookie_monster::{Cookie, Error};

use crate::assert_eq_ser;

#[test]
fn name_value() {
    assert_eq_ser!(Cookie::new("foo", "bar"), Ok("foo=bar"));
    assert_eq_ser!(Cookie::new("foo", ""), Ok("foo="));
    assert_eq_ser!(Cookie::new("foo", "\"\""), Ok("foo=\"\""));
}

#[test]
fn invalid_name_value() {
    assert_eq_ser!(Cookie::new("", "bar"), Err(&Error::NameEmpty));
    assert_eq_ser!(Cookie::new("foo\0", "bar"), Err(&Error::InvalidName));
    assert_eq_ser!(Cookie::new("foo", "bar\0"), Err(&Error::InvalidValue('\0')));
    assert_eq_ser!(Cookie::new("foo", " "), Err(&Error::InvalidValue(' ')));
    assert_eq_ser!(Cookie::new("foo", "\""), Err(&Error::InvalidValue('\"')));
}
