#[macro_export]
macro_rules! assert_eq_ser {
    (
        $cookie:expr,
        strict = $strict:expr,
        relaxed = $relaxed:expr,
        unchecked = $unchecked:expr
    ) => {
        assert_eq!(
            $cookie.serialize_strict().as_deref(),
            $strict,
            "strict went wrong"
        );
        assert_eq!(
            $cookie.serialize_relaxed().as_deref(),
            $relaxed,
            "relaxed went wrong"
        );
        assert_eq!(
            $cookie.serialize_unchecked(),
            $unchecked,
            "unchecked went wrong"
        );
    };
    (
        $cookie:expr,
        strict = $strict:expr,
        unchecked = $unchecked:expr
    ) => {
        assert_eq_ser!(
            $cookie,
            strict = $strict,
            relaxed = $strict,
            unchecked = $unchecked
        );
    };
    ($cookie:expr, all = $all:expr) => {
        assert_eq_ser!(
            $cookie,
            strict = Ok($all),
            relaxed = Ok($all),
            unchecked = $all
        )
    };
}
