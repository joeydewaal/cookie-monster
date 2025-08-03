#[macro_export]
macro_rules! assert_eq_parse {
    (
        $string:expr,
        parse = $relaxed:expr,
        unchecked = $unchecked:expr
    ) => {
        assert_eq!(Cookie::parse($string), $relaxed, "parse went wrong");
        assert_eq!(
            Cookie::parse_unchecked($string),
            $unchecked,
            "unchecked went wrong"
        );
    };
    ($string:expr, all = $all:expr) => {
        assert_eq!(Cookie::parse($string).unwrap(), $all);
        assert_eq!(Cookie::parse_unchecked($string), $all);
    };
}

#[macro_export]
macro_rules! assert_ne_parse {
    (
        $string:expr,
        parse = $relaxed:expr,
        unchecked = $unchecked:expr
    ) => {
        assert_eq!(Cookie::parse($string), $relaxed, "parse went wrong");
        assert_ne!(Cookie::parse($string), $relaxed, "relaxed went wrong");
        assert_ne!(
            Cookie::parse_unchecked($string),
            $unchecked,
            "unchecked went wrong"
        );
    };
    ($string:expr, all = $all:expr) => {
        assert_ne!(Cookie::parse($string).unwrap(), $all);
        assert_ne!(Cookie::parse_unchecked($string), $all);
    };
}

#[macro_export]
macro_rules! assert_eq_parse_unchecked {
    ($string:expr, $expected:expr) => {
        let cookie = Cookie::parse_unchecked($string);
        assert_eq!(cookie, $expected);
    };
}

#[macro_export]
macro_rules! assert_ne_parse_unchecked {
    ($string:expr, $expected:expr) => {
        let cookie = Cookie::parse_unchecked($string);
        assert_ne!(cookie, $expected);
    };
}

#[macro_export]
macro_rules! assert_eq_ser {
    ($cookie:expr, $expected:expr) => {
        let ser = match $cookie.build().serialize() {
            Ok(cookie) => cookie,
            Err(e) => panic!("Failed to serialize {:?}: {:?}", $cookie, e),
        };

        assert_eq!(ser, $expected);
    };
}
