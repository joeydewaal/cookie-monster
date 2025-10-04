#[macro_export]
macro_rules! assert_eq_ser {
    ($string:expr, $cookie:expr) => {
        assert_eq!(
            $string.serialize().as_deref(),
            $cookie,
            "serialize went wrong"
        );
    };
}

#[macro_export]
macro_rules! assert_eq_ser_enc {
    ($string:expr, $cookie:expr) => {
        assert_eq!(
            $string.serialize_encoded().as_deref(),
            $cookie,
            "serialize went wrong"
        );
    };
}
