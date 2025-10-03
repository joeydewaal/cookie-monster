use cookie_monster::{Cookie, SameSite};

use crate::assert_eq_ser;

#[test]
fn same_site() {
    assert_eq_ser!(
        Cookie::build("foo", "bar")
            .same_site(SameSite::Strict)
            .build(),
        Ok("foo=bar; SameSite=Strict")
    );
    assert_eq_ser!(
        Cookie::build("foo", "bar").same_site(SameSite::Lax).build(),
        Ok("foo=bar; SameSite=Lax")
    );
    assert_eq_ser!(
        Cookie::build("foo", "bar")
            .same_site(SameSite::None)
            .build(),
        Ok("foo=bar; Secure; SameSite=None")
    );
}
