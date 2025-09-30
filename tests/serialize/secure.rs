use cookie_monster::Cookie;

mod util;

#[test]
fn secure() {
    assert_eq_ser!(
        Cookie::build("foo", "bar").secure().build(),
        Ok("foo=bar; Secure")
    );
    assert_eq_ser!(
        Cookie::build("foo", "").secure().build(),
        Ok("foo=; Secure")
    );
}
