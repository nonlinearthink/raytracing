extern crate raytracing;

use raytracing::core::{Point3, Ray, Vector3};

#[test]
fn ray_compute() {
    let origin = Point3::new(1., 2., 3.);
    let direction = Vector3::new(4., 5., 6.);
    let ray = Ray::new(origin, direction);

    let target = ray.at(2.);
    assert_eq!(origin, Vector3::new(1., 2., 3.));
    assert_eq!(direction, Vector3::new(4., 5., 6.));
    assert_eq!(target, Vector3::new(9., 12., 15.));
}
