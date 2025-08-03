use cookie_monster::Cookie;

mod util;

#[test]
fn domain() {
    assert_eq_ser!(
        Cookie::build("foo", "bar").domain("rust-lang.com").build(),
        all = "foo=bar; Domain=rust-lang.com"
    );

    assert_eq_ser!(
        Cookie::build("foo", "bar").domain(".rust-lang.com").build(),
        serialize = Ok("foo=bar; Domain=rust-lang.com"),
        unchecked = "foo=bar; Domain=.rust-lang.com"
    );

    assert_eq_ser!(
        Cookie::build("foo", "bar").domain("").build(),
        serialize = Ok("foo=bar"),
        unchecked = "foo=bar; Domain="
    );

    assert_eq_ser!(
        Cookie::build("foo", "bar")
            .domain("rust-lang\0.com")
            .build(),
        serialize = Ok("foo=bar"),
        unchecked = "foo=bar; Domain=rust-lang\0.com"
    );
}
