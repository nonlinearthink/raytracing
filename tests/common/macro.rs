#[macro_export]
macro_rules! assert_vector3_eq {
    ($vec:expr, $x:expr, $y:expr, $z:expr) => {
        assert_eq!($vec.x, $x);
        assert_eq!($vec.y, $y);
        assert_eq!($vec.z, $z);
    };
}
