extern crate tiny_raytracer;

use tiny_raytracer::core::Vector3;

#[test]
fn vector_new_test() {
    let v1 = Vector3::new(1., 2., 3.);
    let v2 = Vector3::zero();

    assert_eq!(v1, Vector3::new(1., 2., 3.));
    assert_eq!(v2, Vector3::new(0., 0., 0.));
}

#[test]
fn vector_random_test() {
    let mut rng = rand::thread_rng();
    for _ in 0..100 {
        let v1 = Vector3::random(0., 1., &mut rng);
        let v2 = Vector3::random(5., 10., &mut rng);

        assert!(v1.x >= 0. && v1.x < 1.);
        assert!(v1.y >= 0. && v1.y < 1.);
        assert!(v1.z >= 0. && v1.z < 1.);

        assert!(v2.x >= 5. && v2.x < 10.);
        assert!(v2.y >= 5. && v2.y < 10.);
        assert!(v2.z >= 5. && v2.z < 10.);
    }
}

#[test]
fn vector_random_in_unit_sphere_test() {
    for _ in 0..100 {
        let v1 = Vector3::random_in_unit_sphere();

        assert!(v1.length() <= 1.);
    }
}

#[test]
fn vector_random_in_unit_disk_test() {
    for _ in 0..100 {
        let v1 = Vector3::random_in_unit_disk();

        assert!(f32::abs(v1.x.powi(2) + v1.y.powi(2)) <= 1. && v1.z == 0.);
    }
}

#[test]
fn vector_random_unit_vector_test() {
    for _ in 0..100 {
        let v1 = Vector3::random_unit_vector();

        assert!(v1.length() <= 1. + f32::EPSILON);
    }
}

#[test]
fn vector_random_on_hemisphere_test() {
    for _ in 0..100 {
        let v1 = Vector3::random_on_hemisphere(&Vector3::new(0., 1., 0.));

        assert!(v1.y >= 0.);
        assert!(v1.length() <= 1. + f32::EPSILON);
    }
}

#[test]
fn vector_dot_test() {
    let v1 = Vector3::new(1., 2., 3.);
    let v2 = Vector3::new(4., 5., 6.);

    let result: f32 = v1.dot(&v2);
    assert_eq!(v1, Vector3::new(1., 2., 3.));
    assert_eq!(v2, Vector3::new(4., 5., 6.));
    assert_eq!(result, 32.);
}

#[test]
fn vector_cross_test() {
    let v1 = Vector3::new(1., 2., 3.);
    let v2 = Vector3::new(4., 5., 6.);

    let result = v1.cross(&v2);
    assert_eq!(v1, Vector3::new(1., 2., 3.));
    assert_eq!(v2, Vector3::new(4., 5., 6.));
    assert_eq!(result, Vector3::new(-3., 6., -3.));
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

    assert_eq!(
        v.normolize(),
        Vector3::new(1. / 2., 1. / 2., f32::sqrt(2.) / 2.)
    );
}

#[test]
fn vector_reflect_test() {
    let v1 = Vector3::new(-1., 2., -3.);
    let normal = Vector3::new(0., 1., 0.);
    let v2 = v1.reflect(&normal);

    assert_eq!(v1, Vector3::new(-1., 2., -3.));
    assert_eq!(normal, Vector3::new(0., 1., 0.));
    assert_eq!(v2, Vector3::new(-1., -2., -3.));
}

#[test]
fn vector_refract_test() {
    let v1 = Vector3::new(-1., 2., -3.);
    let normal = Vector3::new(0., 1., 0.);
    let v2 = v1.refract(&normal, 1.);

    assert_eq!(v1, Vector3::new(-1., 2., -3.));
    assert_eq!(normal, Vector3::new(0., 1., 0.));
    assert_eq!(v2, Vector3::new(-1., -3., -3.));
}

#[test]
fn vector_equals_test() {
    let v1 = Vector3::new(1., 2., 3.);
    let v2 = Vector3::new(1.00000001, 2.00000001, 3.00000001);

    assert!(v1.equals(&v2));
}

#[test]
fn vector_equals_zero_test() {
    let v1 = Vector3::new(0.00000001, -0.00000001, 0.00000001);

    assert!(v1.equals_zero());
}

#[test]
fn vector_index_test() {
    let mut v = Vector3::new(1., 2., 3.);

    assert_eq!(v[0], 1.);
    assert_eq!(v[1], 2.);
    assert_eq!(v[2], 3.);

    v[0] = 2.;

    assert_eq!(v[0], 2.);
}

#[test]
fn vector_add_test() {
    let v1 = Vector3::new(1., 2., 3.);
    let v2 = Vector3::new(4., 5., 6.);

    // add by vector3
    let result = v1 + &v2;
    assert_eq!(v1, Vector3::new(1., 2., 3.));
    assert_eq!(v2, Vector3::new(4., 5., 6.));
    assert_eq!(result, Vector3::new(5., 7., 9.));

    // add by f32
    let result = v1 + 2.;
    assert_eq!(v1, Vector3::new(1., 2., 3.));
    assert_eq!(result, Vector3::new(3., 4., 5.));
}

#[test]
fn vector_sub_test() {
    let v1 = Vector3::new(1., 2., 3.);
    let v2 = Vector3::new(4., 5., 6.);

    // sub by vector3
    let result = v1 - &v2;
    assert_eq!(v1, Vector3::new(1., 2., 3.));
    assert_eq!(v2, Vector3::new(4., 5., 6.));
    assert_eq!(result, Vector3::new(-3., -3., -3.));

    // sub by f32
    let result = v1 - 2.;
    assert_eq!(v1, Vector3::new(1., 2., 3.));
    assert_eq!(result, Vector3::new(-1., 0., 1.));
}

#[test]
fn vector_mul_test() {
    let v1 = Vector3::new(1., 2., 3.);
    let v2 = Vector3::new(4., 5., 6.);

    // mul by vector3
    let result = v1 * &v2;
    assert_eq!(v1, Vector3::new(1., 2., 3.));
    assert_eq!(v2, Vector3::new(4., 5., 6.));
    assert_eq!(result, Vector3::new(4., 10., 18.));

    // mul by f32
    let result = v1 * 2.;
    assert_eq!(v1, Vector3::new(1., 2., 3.));
    assert_eq!(result, Vector3::new(2., 4., 6.));
}

#[test]
fn vector_div_test() {
    let v1 = Vector3::new(1., 2., 3.);

    // div by f32
    let result = v1 / 2.;
    assert_eq!(v1, Vector3::new(1., 2., 3.));
    assert_eq!(result, Vector3::new(0.5, 1., 1.5));
}

#[test]
fn vector_add_assign_test() {
    let mut v1 = Vector3::new(1., 2., 3.);
    let mut v2 = Vector3::new(4., 5., 6.);

    // sub assign by vector3
    v1 += &v2;
    assert_eq!(v1, Vector3::new(5., 7., 9.));
    assert_eq!(v2, Vector3::new(4., 5., 6.));

    // mul assign by f32
    v2 += 2.;
    assert_eq!(v2, Vector3::new(6., 7., 8.));
}

#[test]
fn vector_sub_assign_test() {
    let mut v1 = Vector3::new(1., 2., 3.);
    let mut v2 = Vector3::new(4., 5., 6.);

    // sub assign by vector3
    v1 -= &v2;
    assert_eq!(v1, Vector3::new(-3., -3., -3.));
    assert_eq!(v2, Vector3::new(4., 5., 6.));

    // sub assign by f32
    v2 -= 2.;
    assert_eq!(v2, Vector3::new(2., 3., 4.));
}

#[test]
fn vector_mul_assign_test() {
    let mut v1 = Vector3::new(1., 2., 3.);
    let mut v2 = Vector3::new(4., 5., 6.);

    // mul assign by vector3
    v1 *= &v2;
    assert_eq!(v1, Vector3::new(4., 10., 18.));
    assert_eq!(v2, Vector3::new(4., 5., 6.));

    // mul assign by f32
    v2 *= 2.;
    assert_eq!(v2, Vector3::new(8., 10., 12.));
}

#[test]
fn vector_div_assign_test() {
    let mut v1 = Vector3::new(1., 2., 3.);
    let mut v2 = Vector3::new(4., 5., 6.);

    // div assign by vector3
    v1 /= &v2;
    assert_eq!(v1, Vector3::new(0.25, 0.4, 0.5));
    assert_eq!(v2, Vector3::new(4., 5., 6.));

    // div assign by f32
    v2 /= 2.;
    assert_eq!(v2, Vector3::new(2., 2.5, 3.));
}
