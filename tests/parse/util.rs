#[macro_export]
macro_rules! assert_eq_parse {
    ($string:expr, $cookie:expr) => {
        assert_eq!(Cookie::parse($string), $cookie, "parse went wrong");
    };
}

#[macro_export]
macro_rules! assert_ne_parse {
    ($string:expr, $cookie:expr) => {
        assert_ne!(Cookie::parse($string), $cookie, "parse went wrong");
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
