use std::ops::Div;

use super::{HitRecord, Hittable, Point3, Ray, Vector3};

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
    fn hit(self, ray: &Ray, min_t: f32, max_t: f32, record: &mut HitRecord) -> bool {
        let oc: Vector3 = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = oc.dot(&ray.direction);
        let c = oc.length_squared() - f32::powf(self.radius, 2.);

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0. {
            return false;
        }
        let sqrt_discriminant = f32::sqrt(discriminant);

        let mut root = (-half_b - sqrt_discriminant) / a;
        if root <= min_t || max_t <= root {
            root = (-half_b + sqrt_discriminant) / a;
            if root <= min_t || max_t <= root {
                return false;
            }
        }

        record.t = root;
        record.point = ray.at(record.t);
        record.normal = (&record.point - &self.center).div(self.radius);

        return true;
    }
}
