use cookie_monster::Cookie;

mod util;

#[test]
fn partitioned() {
    assert_eq_ser!(
        Cookie::build("foo", "bar").partitioned().build(),
        all = "foo=bar; Secure; Partitioned"
    );
    assert_eq_ser!(
        Cookie::build("foo", "").partitioned().build(),
        all = "foo=; Secure; Partitioned"
    );
}
