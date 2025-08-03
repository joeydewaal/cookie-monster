use cookie_monster::Cookie;

mod util;

#[test]
fn http_only() {
    assert_eq_parse!(
        " foo=bar; HttpOnly",
        all = Cookie::build("foo", "bar").http_only().build()
    );
    assert_eq_parse!(
        " foo=bar;HttpOnly",
        all = Cookie::build("foo", "bar").http_only().build()
    );
    assert_eq_parse!(
        " foo=bar; httponly",
        all = Cookie::build("foo", "bar").http_only().build()
    );
    assert_eq_parse!(
        " foo=bar;httponly",
        all = Cookie::build("foo", "bar").http_only().build()
    );
    assert_eq_parse!(
        " foo=bar ;HttpOnly",
        all = Cookie::build("foo", "bar").http_only().build()
    );
    assert_eq_parse!(
        " foo=bar ;HttpOnly;;",
        all = Cookie::build("foo", "bar").http_only().build()
    );
}

#[test]
fn invalid_http_only() {
    assert_eq_parse!(
        " foo=bar; HttpOnly\0",
        parse = Ok(Cookie::new("foo", "bar")),
        unchecked = Cookie::new("foo", "bar")
    );
}
