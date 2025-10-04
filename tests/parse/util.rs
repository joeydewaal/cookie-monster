#[macro_export]
macro_rules! assert_eq_parse {
    ($string:expr, $cookie:expr) => {
        assert_eq!(Cookie::parse($string), $cookie, "parse went wrong");
    };
}

#[macro_export]
macro_rules! assert_eq_parse_enc {
    ($string:expr, $cookie:expr) => {
        assert_eq!(Cookie::parse_encoded($string), $cookie, "parse went wrong");
    };
}

#[macro_export]
macro_rules! assert_ne_parse {
    ($string:expr, $cookie:expr) => {
        assert_ne!(Cookie::parse($string), $cookie, "parse went wrong");
    };
}
