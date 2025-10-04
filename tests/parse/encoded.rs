use cookie_monster::Cookie;

use crate::assert_eq_parse_enc;

#[test]
fn encoded() {
    assert_eq_parse_enc!("foo=bar%00", Ok(Cookie::new("foo", "bar\0")));

    assert_eq_parse_enc!(
        "foo%20!%%3F%3D=bar%3B%3B%2C%20a",
        Ok(Cookie::new("foo !%?=", "bar;;, a"))
    );
}
