#[macro_export]
macro_rules! assert_eq_parse {
    (
        $string:expr,
        strict = $strict:expr,
        relaxed = $relaxed:expr,
        unchecked = $unchecked:expr
    ) => {
        assert_eq!(Cookie::parse_strict($string), $strict, "strict went wrong");
        assert_eq!(
            Cookie::parse_relaxed($string),
            $relaxed,
            "relaxed went wrong"
        );
        assert_eq!(
            Cookie::parse_unchecked($string),
            $unchecked,
            "unchecked went wrong"
        );
    };
    (
        $string:expr,
        strict = $strict:expr,
        unchecked = $unchecked:expr
    ) => {
        assert_eq_parse!(
            $string,
            strict = $strict,
            relaxed = $strict,
            unchecked = $unchecked
        );
    };
    ($string:expr, all = $all:expr) => {
        assert_eq!(Cookie::parse_strict($string).unwrap(), $all);
        assert_eq!(Cookie::parse_relaxed($string).unwrap(), $all);
        assert_eq!(Cookie::parse_unchecked($string), $all);
    };
}

#[macro_export]
macro_rules! assert_ne_parse {
    (
        $string:expr,
        strict = $strict:expr,
        relaxed = $relaxed:expr,
        unchecked = $unchecked:expr
    ) => {
        assert_eq!(Cookie::parse_strict($string), $strict, "strict went wrong");
        assert_ne!(
            Cookie::parse_relaxed($string),
            $relaxed,
            "relaxed went wrong"
        );
        assert_ne!(
            Cookie::parse_unchecked($string),
            $unchecked,
            "unchecked went wrong"
        );
    };
    (
        $string:expr,
        strict = $strict:expr,
        unchecked = $unchecked:expr
    ) => {
        assert_ne_parse!(
            $string,
            strict = $strict,
            relaxed = $strict,
            unchecked = $unchecked
        );
    };
    ($string:expr, all = $all:expr) => {
        assert_ne!(Cookie::parse_strict($string).unwrap(), $all);
        assert_ne!(Cookie::parse_relaxed($string).unwrap(), $all);
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
macro_rules! assert_eq_parse_strict {
    ($string:expr, $expected:expr) => {
        let cookie = match Cookie::parse_strict($string) {
            Ok(cookie) => cookie,
            Err(e) => panic!("Failed to parse {:?}: {:?}", $string, e),
        };

        assert_eq!(cookie, $expected);
    };
}

#[macro_export]
macro_rules! assert_ne_parse_strict {
    ($string:expr, $expected:expr) => {
        let cookie = match Cookie::parse_strict($string) {
            Ok(cookie) => cookie,
            Err(e) => panic!("Failed to parse {:?}: {:?}", $string, e),
        };

        assert_ne!(cookie, $expected);
    };
}

#[macro_export]
macro_rules! assert_err_parse_strict {
    ($string:expr, $expected:expr) => {
        let cookie = Cookie::parse_strict($string);
        assert_eq!(cookie, Err($expected));
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
