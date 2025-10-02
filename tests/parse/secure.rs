use cookie_monster::Cookie;

use crate::assert_eq_parse;

#[test]
fn secure() {
    assert_eq_parse!(
        "foo=bar; Secure",
        Ok(Cookie::build("foo", "bar").secure().build())
    );
    assert_eq_parse!(
        "foo=bar; secure",
        Ok(Cookie::build("foo", "bar").secure().build())
    );
    assert_eq_parse!(
        "foo=bar; SECURE",
        Ok(Cookie::build("foo", "bar").secure().build())
    );
}

#[test]
fn invalid_secure() {
    assert_eq_parse!("foo=bar; Sekure", Ok(Cookie::new("foo", "bar")));
    assert_eq_parse!("foo=bar; Secure\0", Ok(Cookie::new("foo", "bar")));
}
