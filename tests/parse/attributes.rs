use cookie_monster::Cookie;

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
        parse = Ok(Cookie::new("foo", "bar")),
        unchecked = Cookie::new("foo", "bar")
    );
    assert_eq_parse!(
        " foo=bar ;HttpOnly; =secure",
        parse = Ok(Cookie::build("foo", "bar").http_only().build()),
        unchecked = Cookie::build("foo", "bar").http_only().build()
    );
    assert_eq_parse!(
        "foo=bar; Sekure",
        parse = Ok(Cookie::new("foo", "bar")),
        unchecked = Cookie::new("foo", "bar")
    );
}
