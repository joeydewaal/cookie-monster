use cookie_monster::{Cookie, Error};

// #[test]
// fn parse_same_site() {
//     let expected = Cookie::build("foo", "bar").same_site(SameSite::Lax);
//     assert_eq_parse_strict!("foo=bar; SameSite=Lax", expected);
//     assert_eq_parse_strict!("foo=bar; SameSite=lax", expected);
//     assert_eq_parse_strict!("foo=bar; SameSite=LAX", expected);
//     assert_eq_parse_strict!("foo=bar; samesite=Lax", expected);
//     assert_eq_parse_strict!("foo=bar; SAMESITE=Lax", expected);

//     let expected = Cookie::build("foo", "bar").same_site(SameSite::Strict);
//     assert_eq_parse_strict!("foo=bar; SameSite=Strict", expected);
//     assert_eq_parse_strict!("foo=bar; SameSITE=Strict", expected);
//     assert_eq_parse_strict!("foo=bar; SameSite=strict", expected);
//     assert_eq_parse_strict!("foo=bar; SameSite=STrICT", expected);
//     assert_eq_parse_strict!("foo=bar; SameSite=STRICT", expected);

//     let expected = Cookie::build("foo", "bar").same_site(SameSite::None);
//     assert_eq_parse_strict!("foo=bar; SameSite=None", expected);
//     assert_eq_parse_strict!("foo=bar; SameSITE=none", expected);
//     assert_eq_parse_strict!("foo=bar; SameSite=NOne", expected);
//     assert_eq_parse_strict!("foo=bar; SameSite=nOne", expected);
// }
