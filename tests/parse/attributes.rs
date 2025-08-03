use cookie_monster::{Cookie, Error};

mod util;

#[test]
fn attributes() {
    let cookie = Cookie::build("foo", "bar").secure().http_only().build();

    assert_eq_parse!("foo=bar; Secure; HttpOnly", all = cookie);
    assert_eq_parse!("foo=bar; Secure;; HttpOnly", all = cookie);
    // assert_eq_parse!("foo=bar; Secure ;; HttpOnly; ", all = cookie);
}

#[test]
fn invalid_attribute() {
    // Invalid characters are skipped.
    assert_eq_parse!(
        "foo=bar; Secure\0",
        strict = Err(Error::InvalidAttribute),
        relaxed = Ok(Cookie::new("foo", "bar")),
        unchecked = Cookie::new("foo", "bar")
    );
    assert_eq_parse!(
        " foo=bar ;HttpOnly; =secure",
        strict = Err(Error::InvalidAttribute),
        // TODO: Should relaxed reject this?
        relaxed = Ok(Cookie::build("foo", "bar").http_only().build()),
        unchecked = Cookie::build("foo", "bar").http_only().build()
    );
    assert_eq_parse!(
        "foo=bar; Sekure",
        strict = Ok(Cookie::new("foo", "bar")),
        unchecked = Cookie::new("foo", "bar")
    );
}
