use cookie_monster::Cookie;

mod util;

#[test]
fn path() {
    assert_eq_parse!(
        "foo=bar; Path=/home",
        all = Cookie::build("foo", "bar").path("/home").build()
    );
    assert_eq_parse!(
        "foo=bar; Path= /home",
        all = Cookie::build("foo", "bar").path("/home").build()
    );
    assert_eq_parse!(
        "foo=bar; Path=/home ",
        all = Cookie::build("foo", "bar").path("/home").build()
    );
    assert_eq_parse!(
        "foo=bar; Path= /home ",
        all = Cookie::build("foo", "bar").path("/home").build()
    );
    assert_eq_parse!(
        "foo=bar; PATH=/home",
        all = Cookie::build("foo", "bar").path("/home").build()
    );
}

#[test]
fn invalid_path() {
    assert_eq_parse!(
        "foo=bar; Path=",
        strict = Ok(Cookie::new("foo", "bar")),
        unchecked = Cookie::build("foo", "bar").path("").build()
    );
    assert_eq_parse!(
        "foo=bar; Path=home",
        strict = Ok(Cookie::new("foo", "bar")),
        unchecked = Cookie::build("foo", "bar").path("home").build()
    );
    assert_eq_parse!(
        "foo=bar; Path=home/",
        strict = Ok(Cookie::new("foo", "bar")),
        unchecked = Cookie::build("foo", "bar").path("home/").build()
    );
    assert_eq_parse!(
        "foo=bar; Path=home/test",
        strict = Ok(Cookie::new("foo", "bar")),
        unchecked = Cookie::build("foo", "bar").path("home/test").build()
    );
    assert_eq_parse!(
        "foo=bar; Path=/home/\0",
        strict = Ok(Cookie::new("foo", "bar")),
        unchecked = Cookie::build("foo", "bar").path("/home/\0").build()
    );
}
