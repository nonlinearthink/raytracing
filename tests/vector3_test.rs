extern crate rst_raytrace;

mod common;

use rst_raytrace::core::Vector3;

#[test]
fn vector_new_test() {
    let v1 = Vector3::new(1., 2., 3.);
    let v2 = Vector3::zero();

    assert_vector3_eq!(v1, 1., 2., 3.);
    assert_vector3_eq!(v2, 0., 0., 0.);
}

#[test]
fn vector_dot_test() {
    let v1 = Vector3::new(1., 2., 3.);
    let v2 = Vector3::new(4., 5., 6.);

    let result: f32 = v1.dot(&v2);
    assert_vector3_eq!(v1, 1., 2., 3.);
    assert_vector3_eq!(v2, 4., 5., 6.);
    assert_eq!(result, 32.);
}

#[test]
fn vector_cross_test() {
    let v1 = Vector3::new(1., 2., 3.);
    let v2 = Vector3::new(4., 5., 6.);

    let result = v1.cross(&v2);
    assert_vector3_eq!(v1, 1., 2., 3.);
    assert_vector3_eq!(v2, 4., 5., 6.);
    assert_vector3_eq!(result, -3., 6., -3.);
}

#[test]
fn vector_length_squared_test() {
    let v = Vector3::new(1., 2., 3.);

    assert_eq!(v.length_squared(), 14.);
}

#[test]
fn vector_length_test() {
    let v = Vector3::new(1., 2., 3.);

    assert_eq!(v.length(), f32::sqrt(14.));
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
    let v1 = Vector3::new(1., 2., 3.);
    let v2 = Vector3::new(4., 5., 6.);

    let result = &v1 + &v2;
    assert_vector3_eq!(v1, 1., 2., 3.);
    assert_vector3_eq!(v2, 4., 5., 6.);
    assert_vector3_eq!(result, 5., 7., 9.);
}

#[test]
fn vector_sub_test() {
    let v1 = Vector3::new(1., 2., 3.);
    let v2 = Vector3::new(4., 5., 6.);

    let result = &v1 - &v2;
    assert_vector3_eq!(v1, 1., 2., 3.);
    assert_vector3_eq!(v2, 4., 5., 6.);
    assert_vector3_eq!(result, -3., -3., -3.);
}

#[test]
fn vector_mul_test() {
    let v1 = Vector3::new(1., 2., 3.);
    let v2 = Vector3::new(4., 5., 6.);

    // mul by vector3
    let result = &v1 * &v2;
    assert_vector3_eq!(v1, 1., 2., 3.);
    assert_vector3_eq!(v2, 4., 5., 6.);
    assert_vector3_eq!(result, 4., 10., 18.);

    // mul by vector3
    let result = &v1 * 2.;
    assert_vector3_eq!(v1, 1., 2., 3.);
    assert_vector3_eq!(result, 2., 4., 6.);
}

#[test]
fn vector_div_test() {
    let v1 = Vector3::new(1., 2., 3.);
    let v2 = Vector3::new(4., 5., 6.);

    // div by vector3
    let result = &v1 / &v2;
    assert_vector3_eq!(v1, 1., 2., 3.);
    assert_vector3_eq!(v2, 4., 5., 6.);
    assert_vector3_eq!(result, 0.25, 0.4, 0.5);

    // div by f32
    let result = &v1 / 2.;
    assert_vector3_eq!(v1, 1., 2., 3.);
    assert_vector3_eq!(result, 0.5, 1., 1.5);
}

#[test]
fn vector_add_assign_test() {
    let mut v1 = Vector3::new(1., 2., 3.);
    let v2 = Vector3::new(4., 5., 6.);

    v1 += &v2;
    assert_vector3_eq!(v1, 5., 7., 9.);
    assert_vector3_eq!(v2, 4., 5., 6.);
}

#[test]
fn vector_sub_assign_test() {
    let mut v1 = Vector3::new(1., 2., 3.);
    let v2 = Vector3::new(4., 5., 6.);

    v1 -= &v2;
    assert_vector3_eq!(v1, -3., -3., -3.);
    assert_vector3_eq!(v2, 4., 5., 6.);
}

#[test]
fn vector_mul_assign_test() {
    let mut v1 = Vector3::new(1., 2., 3.);
    let mut v2 = Vector3::new(4., 5., 6.);

    // mul assign by vector3
    v1 *= &v2;
    assert_vector3_eq!(v1, 4., 10., 18.);
    assert_vector3_eq!(v2, 4., 5., 6.);

    // mul assign by vector3
    v2 *= 2.;
    assert_vector3_eq!(v2, 8., 10., 12.);
}

#[test]
fn vector_div_assign_test() {
    let mut v1 = Vector3::new(1., 2., 3.);
    let mut v2 = Vector3::new(4., 5., 6.);

    // div assign by vector3
    v1 /= &v2;
    assert_vector3_eq!(v1, 0.25, 0.4, 0.5);
    assert_vector3_eq!(v2, 4., 5., 6.);

    // div assign by f32
    v2 /= 2.;
    assert_vector3_eq!(v2, 2., 2.5, 3.);
}
