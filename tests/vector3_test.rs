extern crate rst_raytrace;

use rst_raytrace::math::Vector3;

macro_rules! assert_vector3_eq {
    ($vec:expr, $x:expr, $y:expr, $z:expr) => {
        assert_eq!($vec.x, $x);
        assert_eq!($vec.y, $y);
        assert_eq!($vec.z, $z);
    };
}

#[test]
fn vector_dot_test() {
    let v1 = Vector3::new(1., 2., 4.);
    let v2 = Vector3::new(1., 3., 9.);
    let result: f32 = v1.dot(&v2);
    // the origin vectors not change
    assert_vector3_eq!(v1, 1., 2., 4.);
    assert_vector3_eq!(v2, 1., 3., 9.);
    // the result is correct
    assert_eq!(result, 43.);
}

#[test]
fn vector_cross_test() {
    let v1 = Vector3::new(1., 2., 4.);
    let v2 = Vector3::new(1., 3., 9.);
    let result = v1.cross(&v2);
    // the origin vectors not change
    assert_vector3_eq!(v1, 1., 2., 4.);
    assert_vector3_eq!(v2, 1., 3., 9.);
    // the result is correct
    assert_vector3_eq!(result, 6., -5., 1.);
}
