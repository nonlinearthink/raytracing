extern crate raytracing;

use rand::Rng;
use raytracing::core::{
    HitRecord, Hittable, Interval, LambertianMaterial, Ray, SolidColorTexture, Sphere, Vector2,
    Vector3,
};
use std::rc::Rc;

#[test]
fn sphere_hit_test() {
    let material = Rc::new(LambertianMaterial::new(Rc::new(SolidColorTexture::new(
        0.8, 0.8, 0.0,
    ))));
    let sphere = Sphere::new(Vector3::zero(), 1., material);
    let ray_origin = Vector3::new(0., 2., 0.);

    let ray = Ray::new(ray_origin, Vector3::new(0., -1., 0.));
    let mut record = HitRecord::new();

    let ray_interval = Interval::new(0., 0.999);
    assert!(!sphere.hit(&ray, &ray_interval, &mut record));

    let ray_interval = Interval::new(0., 1.);
    assert!(sphere.hit(&ray, &ray_interval, &mut record));
    assert_eq!(record.normal.unwrap(), Vector3::new(0., 1., 0.));
    assert_eq!(record.point.unwrap(), Vector3::new(0., 1., 0.));

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
fn sphere_uv_test() {
    assert_eq!(
        Sphere::compute_uv(&Vector3::new(1., 0., 0.)),
        Vector2::new(0.5, 0.5)
    );

    assert_eq!(
        Sphere::compute_uv(&Vector3::new(-1., 0., 0.)),
        Vector2::new(0., 0.5)
    );

    assert_eq!(
        Sphere::compute_uv(&Vector3::new(0., 1., 0.)),
        Vector2::new(0.5, 1.)
    );

    assert_eq!(
        Sphere::compute_uv(&Vector3::new(0., -1., 0.)),
        Vector2::new(0.5, 0.)
    );

    assert_eq!(
        Sphere::compute_uv(&Vector3::new(0., 0., 1.)),
        Vector2::new(0.25, 0.5)
    );

    assert_eq!(
        Sphere::compute_uv(&Vector3::new(0., 0., -1.)),
        Vector2::new(0.75, 0.5)
    );
}

#[test]
fn sphere_moving_test() {
    let material = Rc::new(LambertianMaterial::new(Rc::new(SolidColorTexture::new(
        0.8, 0.8, 0.0,
    ))));
    let sphere = Sphere::new_moving_sphere(Vector3::zero(), Vector3::one(), 1., material);

    assert!(sphere.is_moving);
    for _ in 0..5 {
        let mut rng = rand::thread_rng();
        let random = rng.gen::<f32>();
        assert_eq!(
            sphere.get_moving_center(random),
            Vector3::new(random, random, random)
        );
    }
}

#[test]
fn sphere_bounding_box_test() {
    let material = Rc::new(LambertianMaterial::new(Rc::new(SolidColorTexture::new(
        0.8, 0.8, 0.0,
    ))));
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
