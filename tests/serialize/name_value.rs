use cookie_monster::{Cookie, Error};

mod util;

#[test]
fn name_value() {
    assert_eq_ser!(Cookie::new("foo", "bar"), all = "foo=bar");
    assert_eq_ser!(Cookie::new("foo", ""), all = "foo=");
    assert_eq_ser!(Cookie::new("foo", "\"\""), all = "foo=\"\"");
}

#[test]
fn invalid_name_value() {
    assert_eq_ser!(
        Cookie::new("", "bar"),
        strict = Err(&Error::NameEmpty),
        unchecked = "=bar"
    );
    assert_eq_ser!(
        Cookie::new("foo\0", "bar"),
        strict = Err(&Error::InvalidName),
        unchecked = "foo\0=bar"
    );
    assert_eq_ser!(
        Cookie::new("foo", "bar\0"),
        strict = Err(&Error::InvalidValue),
        unchecked = "foo=bar\0"
    );
    assert_eq_ser!(
        Cookie::new("foo", " "),
        strict = Err(&Error::InvalidValue),
        unchecked = "foo= "
    );
    assert_eq_ser!(
        Cookie::new("foo", "\""),
        strict = Err(&Error::InvalidValue),
        unchecked = "foo=\""
    );
}
