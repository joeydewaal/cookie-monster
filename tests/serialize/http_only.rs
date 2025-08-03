use cookie_monster::Cookie;

mod util;

#[test]
fn http_only() {
    assert_eq_ser!(
        Cookie::build("foo", "bar").http_only().build(),
        all = "foo=bar; HttpOnly"
    );
    assert_eq_ser!(
        Cookie::build("foo", "").http_only().build(),
        all = "foo=; HttpOnly"
    );
}
