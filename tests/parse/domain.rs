use cookie_monster::Cookie;

mod util;

#[test]
fn domain() {
    assert_eq_parse!(
        "foo=bar; Domain=domain.com",
        all = Cookie::build("foo", "bar").domain("domain.com").build()
    );
    assert_eq_parse!(
        "foo=bar; domain=domain.com",
        all = Cookie::build("foo", "bar").domain("domain.com").build()
    );
    assert_eq_parse!(
        "foo=bar; DOMAIN=domain.com",
        all = Cookie::build("foo", "bar").domain("domain.com").build()
    );

    assert_eq_parse!(
        "foo=bar; Domain=.domain.com",
        all = Cookie::build("foo", "bar").domain("domain.com").build()
    );
    assert_eq_parse!(
        "foo=bar; Domain=DOMAIN.COM",
        all = Cookie::build("foo", "bar").domain("domain.com").build()
    );
    assert_eq_parse!(
        "foo=bar; Domain=domain.com.",
        all = Cookie::build("foo", "bar").domain("domain.com.").build()
    );
}

#[test]
fn invalid_domain() {
    assert_eq_parse!(
        "foo=bar; Domain=www.foo\0.com",
        strict = Ok(Cookie::new("foo", "bar")),
        unchecked = Cookie::build("foo", "bar").domain("www.foo\0.com").build()
    );
    assert_eq_parse!(
        "foo=bar; Domain=",
        strict = Ok(Cookie::new("foo", "bar")),
        unchecked = Cookie::build("foo", "bar").domain("").build()
    );
}

#[test]
fn domain_not_eq() {
    assert_ne_parse!(
        "foo=bar; Domain=.domain.com",
        all = Cookie::build("foo", "bar").domain(".domain.com").build()
    );
}
