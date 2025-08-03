use cookie_monster::Cookie;

mod util;

#[test]
fn partitioned() {
    assert_eq_parse!(
        " foo=bar; Partitioned",
        all = Cookie::build("foo", "bar").partitioned().build()
    );
    assert_eq_parse!(
        " foo=bar; partitioned",
        all = Cookie::build("foo", "bar").partitioned().build()
    );
    assert_eq_parse!(
        " foo=bar; PARTITIONED",
        all = Cookie::build("foo", "bar").partitioned().build()
    );
}
