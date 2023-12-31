use crate::{
    core::{
        AxisAlignedBoundingBox, HitRecord, Interval, OrthonormalBasis, Point3, Ray, Vector2,
        Vector3,
    },
    traits::{Hittable, Material},
};
use rand::Rng;
use std::{
    ops::{Add, Div, Mul, Sub},
    rc::Rc,
};

#[derive(Debug)]
pub struct Sphere {
    pub center: Point3,
    pub radius: f32,
    pub material: Rc<dyn Material>,
    pub is_moving: bool,
    move_direction: Vector3,
    bbox: AxisAlignedBoundingBox,
}

impl Sphere {
    pub fn new(center: Point3, radius: f32, material: Rc<dyn Material>) -> Self {
        let radius_vec = Vector3::new(radius, radius, radius);
        Self {
            center,
            radius,
            material,
            is_moving: false,
            move_direction: Vector3::zero(),
            bbox: AxisAlignedBoundingBox::new_with_two_points(
                &(&center - &radius_vec),
                &(&center + &radius_vec),
            ),
        }
    }

    pub fn new_moving_sphere(
        center: Point3,
        target: Point3,
        radius: f32,
        material: Rc<dyn Material>,
    ) -> Self {
        let radius_vec = Vector3::new(radius, radius, radius);

        Self {
            center,
            radius,
            material,
            is_moving: true,
            move_direction: &target - &center,
            bbox: AxisAlignedBoundingBox::new_with_two_points(
                &(&center - &radius_vec),
                &(&center + &radius_vec),
            )
            .merge(&AxisAlignedBoundingBox::new_with_two_points(
                &(&target - &radius_vec),
                &(&target + &radius_vec),
            )),
        }
    }

    pub fn get_moving_center(&self, time: f32) -> Vector3 {
        self.center.add(&self.move_direction.mul(time))
    }

    pub fn compute_uv(point: &Point3) -> Vector2 {
        let theta = f32::acos(-point.y);
        let phi = f32::atan2(-point.z, point.x) + std::f32::consts::PI;

        Vector2::new(
            phi / (2. * std::f32::consts::PI),
            theta / std::f32::consts::PI,
        )
    }

    fn random_to_sphere(radius: f32, distance_squared: f32) -> Vector3 {
        let mut rng = rand::thread_rng();
        let r1 = rng.gen::<f32>();
        let r2 = rng.gen::<f32>();
        let z = 1. + r2 * (f32::sqrt(1. - radius * radius / distance_squared) - 1.);

        let phi = 2. * std::f32::consts::PI * r1;
        let x = f32::cos(phi) * f32::sqrt(1. - z * z);
        let y = f32::sin(phi) * f32::sqrt(1. - z * z);

        Vector3::new(x, y, z)
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_interval: &Interval, record: &mut HitRecord) -> bool {
        let center: Point3 = if self.is_moving {
            self.get_moving_center(ray.time)
        } else {
            self.center
        };
        let oc: Vector3 = &ray.origin - &center;
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
        record.uv = Some(Sphere::compute_uv(&outward_normal));
        record.material = Some(Rc::clone(&self.material));

        return true;
    }

    fn bounding_box(&self) -> &AxisAlignedBoundingBox {
        &self.bbox
    }

    fn pdf_value(&self, origin: &Vector3, direction: &Vector3) -> f32 {
        // This method only works for stationary spheres.
        let mut record = HitRecord::new();
        if !self.hit(
            &Ray::new(*origin, *direction),
            &Interval::new(0.001, f32::INFINITY),
            &mut record,
        ) {
            return 0.;
        }

        let cos_theta_max =
            f32::sqrt(1. - self.radius * self.radius / (&self.center - origin).length_squared());
        let solid_angle = 2. * std::f32::consts::PI * (1. - cos_theta_max);

        1. / solid_angle
    }

    fn random(&self, origin: &Vector3) -> Vector3 {
        let direction = &self.center - origin;
        let distance_squared = direction.length_squared();
        let onb = OrthonormalBasis::new_with_w(&direction);
        onb.local(&Sphere::random_to_sphere(self.radius, distance_squared))
    }
}
