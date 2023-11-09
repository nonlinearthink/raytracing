use std::ops::{Div, Sub};

use super::{
    AxisAlignedBoundingBox, HitRecord, Hittable, Interval, Material, Point3, Ray, Vector3,
};

#[derive(Debug, Clone)]
pub struct Sphere {
    pub center: Vector3,
    pub radius: f32,
    pub material: Box<dyn Material>,
    pub is_moving: bool,
    motion_direction: Vector3,
    aabb: AxisAlignedBoundingBox,
}

impl Sphere {
    pub fn new(center: Point3, radius: f32, material: Box<dyn Material>) -> Sphere {
        let radius_vec = Vector3::new(radius, radius, radius);
        Sphere {
            center,
            radius,
            material,
            is_moving: false,
            motion_direction: Vector3::zero(),
            aabb: AxisAlignedBoundingBox::from_bounding_vector(
                &(center - &radius_vec),
                &(center + &radius_vec),
            ),
        }
    }

    pub fn new_moving_sphere(
        center: Point3,
        target: Point3,
        radius: f32,
        material: Box<dyn Material>,
    ) -> Sphere {
        let radius_vec = Vector3::new(radius, radius, radius);
        let aabb1 =
            AxisAlignedBoundingBox::from_bounding_vector(&(center - &radius_vec), &(center + &radius_vec));
        let aabb2 =
            AxisAlignedBoundingBox::from_bounding_vector(&(target - &radius_vec), &(target + &radius_vec));
        let aabb = aabb1.merge(&aabb2);

        Sphere {
            center,
            radius,
            material,
            is_moving: true,
            motion_direction: target - &center,
            aabb,
        }
    }

    pub fn center_after_move(&self, time: f32) -> Vector3 {
        self.center + &(self.motion_direction * time)
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_interval: &Interval, record: &mut HitRecord) -> bool {
        let center: Point3 = if self.is_moving {
            self.center_after_move(ray.time)
        } else {
            self.center
        };
        let oc: Vector3 = ray.origin - &center;
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
            .sub(&center)
            .div(self.radius);
        record.set_face_normal(&ray, &outward_normal);
        record.material = Some(Box::clone(&self.material));

        return true;
    }

    fn bounding_box(&self) -> &AxisAlignedBoundingBox {
        &self.aabb
    }
}
