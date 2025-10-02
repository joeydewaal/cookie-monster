use cookie_monster::Cookie;

use crate::assert_eq_parse;

#[test]
fn domain() {
    let domain = || Ok(Cookie::build("foo", "bar").domain("domain.com").build());

    assert_eq_parse!("foo=bar; Domain=domain.com", domain());
    assert_eq_parse!("foo=bar; domain=domain.com", domain());
    assert_eq_parse!("foo=bar; DOMAIN=domain.com", domain());
    assert_eq_parse!("foo=bar; Domain=.domain.com", domain());
    assert_eq_parse!("foo=bar; Domain=DOMAIN.COM", domain());
}

#[test]
fn domain_trailing_dot() {
    assert_eq_parse!(
        "foo=bar; Domain=domain.com.",
        Ok(Cookie::build("foo", "bar").domain("domain.com.").build())
    );
}

#[test]
fn invalid_domain() {
    assert_eq_parse!(
        "foo=bar; Domain=www.foo\0.com",
        Ok(Cookie::new("foo", "bar"))
    );

    assert_eq_parse!("foo=bar; Domain=", Ok(Cookie::new("foo", "bar")));
}
