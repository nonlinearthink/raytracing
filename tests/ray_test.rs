extern crate rst_raytrace;

mod common;

use rst_raytrace::core::{Ray, Vector3};

#[test]
fn ray_compute() {
    let origin = Vector3::new(1., 2., 3.);
    let direction = Vector3::new(4., 5., 6.);
    let ray = Ray::new(origin, direction);

    let target = ray.at(2.);
    assert_vector3_eq!(origin, 1., 2., 3.);
    assert_vector3_eq!(direction, 4., 5., 6.);
    assert_vector3_eq!(target, 9., 12., 15.);
}
