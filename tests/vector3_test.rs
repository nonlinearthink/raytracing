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

#[test]
fn vector_length_squared_test() {
    let v = Vector3::new(1., 2., 2.);
    assert_eq!(v.length_squared(), 9.);
}

#[test]
fn vector_length_test() {
    let v = Vector3::new(1., 2., 2.);
    assert_eq!(v.length(), 3.);
}

#[test]
fn vector_normolize_test() {
    let v = Vector3::new(1., 1., f32::sqrt(2.));
    assert_vector3_eq!(v.normolize(), 1. / 2., 1. / 2., f32::sqrt(2.) / 2.);
}

#[test]
fn vector_index_test() {
    let v = Vector3::new(1., 2., 3.);
    assert_eq!(v[0], 1.);
    assert_eq!(v[1], 2.);
    assert_eq!(v[2], 3.);
}

#[test]
fn vector_add_test() {
    let v1 = Vector3::new(1., 2., 4.);
    let v2 = Vector3::new(1., 3., 9.);
    let result = &v1 + &v2;
    // the origin vectors not change
    assert_vector3_eq!(v1, 1., 2., 4.);
    assert_vector3_eq!(v2, 1., 3., 9.);
    // the result is correct
    assert_vector3_eq!(result, 2., 5., 13.);
}

#[test]
fn vector_sub_test() {
    let v1 = Vector3::new(1., 2., 4.);
    let v2 = Vector3::new(1., 3., 9.);
    let result = &v2 - &v1;
    // the origin vectors not change
    assert_vector3_eq!(v1, 1., 2., 4.);
    assert_vector3_eq!(v2, 1., 3., 9.);
    // the result is correct
    assert_vector3_eq!(result, 0., 1., 5.);
}

#[test]
fn vector_mul_test() {
    let v1 = Vector3::new(1., 2., 4.);
    let v2 = Vector3::new(1., 3., 9.);
    let result = &v1 * &v2;
    // the origin vectors not change
    assert_vector3_eq!(v1, 1., 2., 4.);
    assert_vector3_eq!(v2, 1., 3., 9.);
    // the result is correct
    assert_vector3_eq!(result, 1., 6., 36.);
}

#[test]
fn vector_div_test() {
    let v1 = Vector3::new(1., 2., 4.);
    let v2 = Vector3::new(1., 3., 9.);
    let result = &v2 / &v1;
    // the origin vectors not change
    assert_vector3_eq!(v1, 1., 2., 4.);
    assert_vector3_eq!(v2, 1., 3., 9.);
    // the result is correct
    assert_vector3_eq!(result, 1., 1.5, 2.25);
}
