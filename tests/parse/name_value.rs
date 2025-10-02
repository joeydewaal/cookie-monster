use cookie_monster::{Cookie, Error};

use crate::{assert_eq_parse, assert_ne_parse};

#[test]
fn name_value() {
    let foo_bar = Ok(Cookie::new("foo", "bar"));

    assert_eq_parse!("foo=bar", foo_bar);
    assert_eq_parse!("foo=bar;", foo_bar);
    assert_eq_parse!("foo=bar;;", foo_bar);
    assert_eq_parse!("foo=bar; ; ", foo_bar);
    assert_eq_parse!("foo=bar ; ; ", foo_bar);
    assert_eq_parse!("foo=bar", foo_bar);
    assert_eq_parse!("  foo=bar", foo_bar);
    assert_eq_parse!("foo  =bar", foo_bar);
    assert_eq_parse!("foo=  bar", foo_bar);
    assert_eq_parse!("foo=bar  ", foo_bar);
    assert_eq_parse!("  foo  =  bar  ", foo_bar);
    assert_eq_parse!("foo=\"bar\"", foo_bar);

    assert_eq_parse!("foo=bar=foo", Ok(Cookie::new("foo", "bar=foo")));
    assert_eq_parse!("foo=", Ok(Cookie::new("foo", "")));
    assert_eq_parse!("FOO=BAR", Ok(Cookie::new("FOO", "BAR")));

    assert_eq_parse!("foo=b%2Fr", Ok(Cookie::new("foo", "b%2Fr")));
}

#[test]
fn invalid_name_value() {
    assert_eq_parse!("foobar", Err(Error::EqualsNotFound));
    assert_eq_parse!("foo&bar", Err(Error::EqualsNotFound));

    assert_eq_parse!("", Err(Error::EqualsNotFound));

    assert_eq_parse!("=bar", Err(Error::NameEmpty));
    assert_eq_parse!(" =bar", Err(Error::NameEmpty));
    assert_eq_parse!("foo=\0", Err(Error::InvalidValue));
    assert_eq_parse!("foo=test\0test", Err(Error::InvalidValue));
}

#[test]
fn name_value_brackets_spaces() {
    assert_eq_parse!("foo=\"bar\"", Ok(Cookie::new("foo", "bar")));

    assert_eq_parse!("foo=\"  bar  \"", Err(Error::InvalidValue));

    assert_eq_parse!("foo=\"", Err(Error::InvalidValue));
    assert_eq_parse!("foo=\"\"bar\"\"", Err(Error::InvalidValue));
    assert_eq_parse!("foo=\"bar", Err(Error::InvalidValue));
    assert_eq_parse!("foo=\"\"bar", Err(Error::InvalidValue));
    assert_eq_parse!("foo=bar\"", Err(Error::InvalidValue));
    assert_eq_parse!("foo=bar\"\"", Err(Error::InvalidValue));
    assert_eq_parse!("foo=\"bar\"\"", Err(Error::InvalidValue));
    assert_eq_parse!("foo=\"  bar\"\"", Err(Error::InvalidValue));
    assert_eq_parse!("foo=\"  bar\"  \"  ", Err(Error::InvalidValue));
    assert_eq_parse!(" foo=\"bar   \" ", Err(Error::InvalidValue));
}

#[test]
fn name_value_brackets_spaces_not_eq() {
    assert_ne_parse!("foo=bar", Ok(Cookie::new("foo", "\"bar\"")));
}

// #[test]
// fn odd_characters_encoded() {
//     let expected = Cookie::new("foo", "b/r");
//     let cookie = match Cookie::parse_encoded("foo=b%2Fr") {
//         Ok(cookie) => cookie,
//         Err(e) => panic!("Failed to parse: {:?}", e)
//     };

//     assert_eq!(cookie, expected);
// }
