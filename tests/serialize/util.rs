#[macro_export]
macro_rules! assert_eq_ser {
    (
        $cookie:expr,
        serialize = $relaxed:expr,
        unchecked = $unchecked:expr
    ) => {
        assert_eq!(
            $cookie.serialize().as_deref(),
            $relaxed,
            "serialize went wrong"
        );
        assert_eq!(
            $cookie.serialize_unchecked(),
            $unchecked,
            "unchecked went wrong"
        );
    };
    ($cookie:expr, all = $all:expr) => {
        assert_eq_ser!($cookie, serialize = Ok($all), unchecked = $all)
    };
}
