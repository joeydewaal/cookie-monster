use cookie_monster::Cookie;

use crate::{assert_eq_parse, assert_ne_parse};

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
    assert_ne_parse!(
        " foo=bar",
        Ok(Cookie::build("foo", "bar").partitioned().build())
    );
}
