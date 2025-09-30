use cookie_monster::Cookie;

mod util;

#[test]
fn partitioned() {
    assert_eq_parse!(
        " foo=bar; Partitioned",
        Ok(Cookie::build("foo", "bar").partitioned().build())
    );
    assert_eq_parse!(
        " foo=bar; partitioned",
        Ok(Cookie::build("foo", "bar").partitioned().build())
    );
    assert_eq_parse!(
        " foo=bar; PARTITIONED",
        Ok(Cookie::build("foo", "bar").partitioned().build())
    );
}
