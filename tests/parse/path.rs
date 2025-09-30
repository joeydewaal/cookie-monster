use cookie_monster::Cookie;

mod util;

#[test]
fn path() {
    assert_eq_parse!(
        "foo=bar; Path=/home",
        Ok(Cookie::build("foo", "bar").path("/home").build())
    );
    assert_eq_parse!(
        "foo=bar; Path= /home",
        Ok(Cookie::build("foo", "bar").path("/home").build())
    );
    assert_eq_parse!(
        "foo=bar; Path=/home ",
        Ok(Cookie::build("foo", "bar").path("/home").build())
    );
    assert_eq_parse!(
        "foo=bar; Path= /home ",
        Ok(Cookie::build("foo", "bar").path("/home").build())
    );
    assert_eq_parse!(
        "foo=bar; PATH=/home",
        Ok(Cookie::build("foo", "bar").path("/home").build())
    );
}

#[test]
fn invalid_path() {
    assert_eq_parse!("foo=bar; Path=", Ok(Cookie::new("foo", "bar")));
    assert_eq_parse!("foo=bar; Path=home", Ok(Cookie::new("foo", "bar")));
    assert_eq_parse!("foo=bar; Path=home/", Ok(Cookie::new("foo", "bar")));
    assert_eq_parse!("foo=bar; Path=home/test", Ok(Cookie::new("foo", "bar")));
    assert_eq_parse!("foo=bar; Path=/home/\0", Ok(Cookie::new("foo", "bar")));
}
