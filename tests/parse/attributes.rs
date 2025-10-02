use cookie_monster::Cookie;

use crate::assert_eq_parse;

#[test]
fn attributes() {
    let cookie = || Cookie::build("foo", "bar").secure().http_only().build();

    assert_eq_parse!("foo=bar; Secure; HttpOnly", Ok(cookie()));
    assert_eq_parse!("foo=bar; Secure;; HttpOnly", Ok(cookie()));
    // assert_eq_parse!("foo=bar; Secure ;; HttpOnly; ", all = cookie);
}

#[test]
fn invalid_attribute() {
    // Invalid characters are skipped.
    assert_eq_parse!("foo=bar; Secure\0", Ok(Cookie::new("foo", "bar")));
    assert_eq_parse!(
        " foo=bar ;HttpOnly; =secure",
        Ok(Cookie::build("foo", "bar").http_only().build())
    );
    assert_eq_parse!("foo=bar; Sekure", Ok(Cookie::new("foo", "bar")));
}
