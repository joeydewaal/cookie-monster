use cookie_monster::{Cookie, Error};

mod util;

#[test]
fn name_value() {
    let foo_bar = Cookie::new("foo", "bar");

    assert_eq_parse!("foo=bar", all = foo_bar);
    assert_eq_parse!("foo=bar;", all = foo_bar);
    assert_eq_parse!("foo=bar;;", all = foo_bar);
    assert_eq_parse!("foo=bar; ; ", all = foo_bar);
    assert_eq_parse!("foo=bar ; ; ", all = foo_bar);
    assert_eq_parse!("foo=bar", all = foo_bar);
    assert_eq_parse!("  foo=bar", all = foo_bar);
    assert_eq_parse!("foo  =bar", all = foo_bar);
    assert_eq_parse!("foo=  bar", all = foo_bar);
    assert_eq_parse!("foo=bar  ", all = foo_bar);
    assert_eq_parse!("  foo  =  bar  ", all = foo_bar);
    assert_eq_parse!("foo=\"bar\"", all = foo_bar);

    assert_eq_parse!("foo=bar=foo", all = Cookie::new("foo", "bar=foo"));
    assert_eq_parse!("foo=", all = Cookie::new("foo", ""));
    assert_eq_parse!("FOO=BAR", all = Cookie::new("FOO", "BAR"));

    assert_eq_parse_unchecked!("foo&bar", Cookie::new("", ""));
    assert_eq_parse_unchecked!("=bar", Cookie::new("", "bar"));
    assert_eq_parse_unchecked!("foo\0=bar", Cookie::new("foo\0", "bar"));
    assert_eq_parse_unchecked!("foo=bar\0", Cookie::new("foo", "bar\0"));

    assert_eq_parse!("foo=b%2Fr", all = Cookie::new("foo", "b%2Fr"));
}

#[test]
fn invalid_name_value() {
    assert_eq_parse!(
        "foobar",
        parse = Err(Error::EqualsNotFound),
        unchecked = Cookie::new("", "")
    );
    assert_eq_parse!(
        "foo&bar",
        parse = Err(Error::EqualsNotFound),
        unchecked = Cookie::new("", "")
    );

    assert_eq_parse!(
        "",
        parse = Err(Error::EqualsNotFound),
        unchecked = Cookie::new("", "")
    );

    assert_eq_parse!(
        "=bar",
        parse = Err(Error::NameEmpty),
        unchecked = Cookie::new("", "bar")
    );
    assert_eq_parse!(
        " =bar",
        parse = Err(Error::NameEmpty),
        unchecked = Cookie::new("", "bar")
    );
    assert_eq_parse!(
        "foo=\0",
        parse = Err(Error::InvalidValue),
        unchecked = Cookie::new("foo", "\0")
    );
    assert_eq_parse!(
        "foo=test\0test",
        parse = Err(Error::InvalidValue),
        unchecked = Cookie::new("foo", "test\0test")
    );
}

#[test]
fn name_value_brackets_spaces() {
    assert_eq_parse!("foo=\"bar\"", all = Cookie::new("foo", "bar"));

    assert_eq_parse!(
        "foo=\"  bar  \"",
        parse = Err(Error::InvalidValue),
        unchecked = Cookie::new("foo", "  bar  ")
    );

    assert_eq_parse!(
        "foo=\"",
        parse = Err(Error::InvalidValue),
        unchecked = Cookie::new("foo", "\"")
    );
    assert_eq_parse!(
        "foo=\"\"bar\"\"",
        parse = Err(Error::InvalidValue),
        unchecked = Cookie::new("foo", "\"bar\"")
    );
    assert_eq_parse!(
        "foo=\"bar",
        parse = Err(Error::InvalidValue),
        unchecked = Cookie::new("foo", "\"bar")
    );
    assert_eq_parse!(
        "foo=\"\"bar",
        parse = Err(Error::InvalidValue),
        unchecked = Cookie::new("foo", "\"\"bar")
    );
    assert_eq_parse!(
        "foo=bar\"",
        parse = Err(Error::InvalidValue),
        unchecked = Cookie::new("foo", "bar\"")
    );
    assert_eq_parse!(
        "foo=bar\"\"",
        parse = Err(Error::InvalidValue),
        unchecked = Cookie::new("foo", "bar\"\"")
    );
    assert_eq_parse!(
        "foo=\"bar\"\"",
        parse = Err(Error::InvalidValue),
        unchecked = Cookie::new("foo", "bar\"")
    );
    assert_eq_parse!(
        "foo=\"  bar\"\"",
        parse = Err(Error::InvalidValue),
        unchecked = Cookie::new("foo", "  bar\"")
    );
    assert_eq_parse!(
        "foo=\"  bar\"  \"  ",
        parse = Err(Error::InvalidValue),
        unchecked = Cookie::new("foo", "  bar\"  ")
    );
    assert_eq_parse!(
        " foo=\"bar   \" ",
        parse = Err(Error::InvalidValue),
        unchecked = Cookie::new("foo", "bar   ")
    );
}

#[test]
fn name_value_brackets_spaces_not_eq() {
    assert_ne_parse!("foo=bar", all = Cookie::new("foo", "\"bar\""));
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
