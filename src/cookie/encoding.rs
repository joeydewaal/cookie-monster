use crate::Error;
use percent_encoding::{AsciiSet, CONTROLS, percent_decode, percent_encode};

use std::borrow::Cow;

pub(crate) fn decode_name_value<'a>(
    name: &'a str,
    val: &'a str,
) -> crate::Result<(Cow<'a, str>, Cow<'a, str>)> {
    Ok((decode(name)?, decode(val)?))
}

fn decode(value: &str) -> crate::Result<Cow<'_, str>> {
    percent_decode(value.as_bytes())
        .decode_utf8()
        .map_err(|_| Error::PercentDecodeError)
}

// %x21 / %x23-2B / %x2D-3A / %x3C-5B / %x5D-7E
// ; US-ASCII characters excluding CTLs,
// ; whitespace DQUOTE, comma, semicolon,
// ; and backslash
const FORBIDDEN_VALUE: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b',').add(b';').add(b'\\');

// <token, defined in [RFC2616], Section 2.2>
// Same as cookie-rs except for %, ^ and |
const FORBIDDEN_NAME: &AsciiSet = &FORBIDDEN_VALUE
    .add(b'\t')
    .add(b'(')
    .add(b')')
    .add(b'<')
    .add(b'>')
    .add(b'@')
    .add(b':')
    .add(b'/')
    .add(b'[')
    .add(b']')
    .add(b'?')
    .add(b'=')
    .add(b'{')
    .add(b'}');

pub fn encode_name(string: &str) -> impl std::fmt::Display + '_ {
    percent_encode(string.as_bytes(), FORBIDDEN_NAME)
}

pub fn encode_value(string: &str) -> impl std::fmt::Display + '_ {
    percent_encode(string.as_bytes(), FORBIDDEN_VALUE)
}
