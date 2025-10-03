use cookie_monster::{Cookie, SameSite};

use crate::{assert_eq_parse, assert_ne_parse};

#[test]
fn parse_same_site() {
    let expected = Ok(Cookie::build("foo", "bar").same_site(SameSite::Lax).build());
    assert_eq_parse!("foo=bar; SameSite=Lax", expected);
    assert_eq_parse!("foo=bar; SameSite=lax", expected);
    assert_eq_parse!("foo=bar; SameSite=LAX", expected);
    assert_eq_parse!("foo=bar; samesite=Lax", expected);
    assert_eq_parse!("foo=bar; SAMESITE=Lax", expected);

    let expected = Ok(Cookie::build("foo", "bar")
        .same_site(SameSite::Strict)
        .build());
    assert_eq_parse!("foo=bar; SameSite=Strict", expected);
    assert_eq_parse!("foo=bar; SameSITE=Strict", expected);
    assert_eq_parse!("foo=bar; SameSite=strict", expected);
    assert_eq_parse!("foo=bar; SameSite=STrICT", expected);
    assert_eq_parse!("foo=bar; SameSite=STRICT", expected);

    let expected = Ok(Cookie::build("foo", "bar")
        .same_site(SameSite::None)
        .build());
    assert_eq_parse!("foo=bar; SameSite=None", expected);
    assert_eq_parse!("foo=bar; SameSITE=none", expected);
    assert_eq_parse!("foo=bar; SameSite=NOne", expected);
    assert_eq_parse!("foo=bar; SameSite=nOne", expected);
}

#[test]
fn eq() {
    let expected = Ok(Cookie::build("foo", "bar")
        .same_site(SameSite::None)
        .build());
    assert_ne_parse!("foo=bar", expected);
}
