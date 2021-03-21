#[macro_export]
/// helper for testing parsers
macro_rules! assert_parser {
    ($call:expr, $expected:expr) => {
        assert_eq!($call.unwrap().1, $expected);
    };
}
