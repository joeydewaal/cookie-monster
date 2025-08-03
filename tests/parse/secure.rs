use cookie_monster::{Cookie, Error};

mod util;

#[test]
fn secure() {
    assert_eq_parse!(
        "foo=bar; Secure",
        all = Cookie::build("foo", "bar").secure().build()
    );
    assert_eq_parse!(
        "foo=bar; secure",
        all = Cookie::build("foo", "bar").secure().build()
    );
    assert_eq_parse!(
        "foo=bar; SECURE",
        all = Cookie::build("foo", "bar").secure().build()
    );
}

#[test]
fn invalid_secure() {
    assert_eq_parse!(
        "foo=bar; Sekure",
        strict = Ok(Cookie::new("foo", "bar")),
        unchecked = Cookie::new("foo", "bar")
    );
    assert_eq_parse!(
        "foo=bar; Secure\0",
        strict = Err(Error::InvalidAttribute),
        relaxed = Ok(Cookie::new("foo", "bar")),
        unchecked = Cookie::new("foo", "bar")
    );
}
