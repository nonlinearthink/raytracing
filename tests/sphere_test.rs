mod common;
extern crate rst_raytrace;

use rand::Rng;
use rst_raytrace::core::{
    Color3, HitRecord, Hittable, Interval, LambertianMaterial, Ray, Sphere, Vector3,
};

#[test]
fn sphere_hit_test() {
    let material = Box::new(LambertianMaterial::new(Some(Color3::new(0.8, 0.8, 0.0))));
    let sphere = Sphere::new(Vector3::zero(), 1., material);
    let ray_origin = Vector3::new(0., 2., 0.);

    let ray = Ray::new(ray_origin, Vector3::new(0., -1., 0.));
    let mut record = HitRecord::new();

    let ray_interval = Interval::new(0., 0.999);
    assert!(!sphere.hit(&ray, &ray_interval, &mut record));

    let ray_interval = Interval::new(0., 1.);
    assert!(sphere.hit(&ray, &ray_interval, &mut record));
    assert_vector3_eq!(record.normal.unwrap(), 0., 1., 0.);
    assert_vector3_eq!(record.point.unwrap(), 0., 1., 0.);

    let ray_interval = Interval::new(0., f32::INFINITY);

    let ray = Ray::new(
        ray_origin,
        Vector3::new(-1., -f32::sqrt(2.999), 0.).normolize(),
    );
    assert!(!sphere.hit(&ray, &ray_interval, &mut record));

    let ray = Ray::new(
        ray_origin,
        Vector3::new(-1., -f32::sqrt(3.), 0.).normolize(),
    );
    assert!(sphere.hit(&ray, &ray_interval, &mut record));
}

#[test]
fn sphere_moving_test() {
    let material = Box::new(LambertianMaterial::new(Some(Color3::new(0.8, 0.8, 0.0))));
    let sphere = Sphere::new_moving_sphere(Vector3::zero(), Vector3::one(), 1., material);

    assert!(sphere.is_moving);
    for _ in 0..5 {
        let mut rng = rand::thread_rng();
        let random = rng.gen::<f32>();
        assert_vector3_eq!(sphere.center_after_move(random), random, random, random);
    }
}

#[test]
fn sphere_bounding_box_test() {
    let material = Box::new(LambertianMaterial::new(Some(Color3::new(0.8, 0.8, 0.0))));
    let sphere = Sphere::new(Vector3::new(2., 2., 2.), 1., material.clone());
    let moving_sphere =
        Sphere::new_moving_sphere(Vector3::zero(), Vector3::one(), 1., material.clone());

    for n in 0..3 {
        assert_eq!(sphere.bounding_box().axis(n).min, 1.);
        assert_eq!(sphere.bounding_box().axis(n).max, 3.);
        assert_eq!(moving_sphere.bounding_box().axis(n).min, -1.);
        assert_eq!(moving_sphere.bounding_box().axis(n).max, 2.);
    }
}
