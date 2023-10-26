use std::ops::{Div, Sub};

use super::{HitRecord, Hittable, Interval, Point3, Ray, Vector3};

#[derive(Debug)]
pub struct Sphere {
    pub center: Vector3,
    pub radius: f32,
}

impl Sphere {
    pub fn new(center: Point3, radius: f32) -> Sphere {
        Sphere { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_interval: &Interval, record: &mut HitRecord) -> bool {
        let oc: Vector3 = ray.origin - &self.center;
        let a = ray.direction.length_squared();
        let half_b = oc.dot(&ray.direction);
        let c = oc.length_squared() - f32::powf(self.radius, 2.);

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0. {
            return false;
        }
        let sqrt_discriminant = f32::sqrt(discriminant);

        let mut root = (-half_b - sqrt_discriminant) / a;
        if !ray_interval.contains(root) {
            root = (-half_b + sqrt_discriminant) / a;
            if !ray_interval.contains(root) {
                return false;
            }
        }

        record.t = root;
        record.point = Some(ray.at(record.t));
        let outward_normal = record
            .point
            .expect("Ray should always have some value at t.")
            .sub(&self.center)
            .div(self.radius);
        record.set_face_normal(&ray, &outward_normal);

        return true;
    }
}
