use crate::{
    core::{AxisAlignedBoundingBox, HitRecord, Interval, Point3, Ray, Vector2, Vector3},
    traits::{Hittable, Material},
};
use rand::Rng;
use std::{
    ops::{Add, Mul},
    rc::Rc,
};

#[derive(Debug)]
pub struct Quad {
    /// A point in space, form a parallelogram plane with two boundary vectors u and v
    origin: Point3,

    /// One of the boundary vector of a parallelogram plane.
    u: Vector3,

    /// Another one of the boundary vector of a parallelogram plane.
    v: Vector3,

    /// Material of this plane.
    material: Rc<dyn Material>,

    /// Bounding box of this plane.
    bbox: AxisAlignedBoundingBox,

    /// A vector perpendicular to this plane, can be used with `point` to form the implicit
    /// formula of the borderless plane which this parallelogram plane lies.
    normal: Vector3,

    /// A precalculated number be defined by the implicit formula of the borderless plane: `ax + by + cz + d = 0``.
    d: f32,

    /// A precalculated number that accelerates the UV computation of the ray-plane intersection.
    w: Vector3,

    /// The area of this parallelogram plane.
    area: f32,
}

impl Quad {
    pub fn new(origin: Point3, u: Vector3, v: Vector3, material: Rc<dyn Material>) -> Self {
        let n = u.cross(&v);
        let normal = n.normolize();
        let d = normal.dot(&origin);
        let w = &n / n.dot(&n);
        let area = n.length();
        Self {
            origin,
            u,
            v,
            material,
            bbox: AxisAlignedBoundingBox::new_with_two_points(&origin, &origin.add(&u).add(&v))
                .pad(),
            normal,
            d,
            w,
            area,
        }
    }
}

impl Hittable for Quad {
    fn hit(
        &self,
        ray: &crate::core::Ray,
        ray_interval: &crate::core::Interval,
        record: &mut crate::core::HitRecord,
    ) -> bool {
        let nd = self.normal.dot(&ray.direction);
        if f32::abs(nd) < f32::EPSILON {
            // The ray is parallel to the plane.
            return false;
        }

        let t = (self.d - self.normal.dot(&ray.origin)) / nd;
        if !ray_interval.contains(t) {
            // The hit point parameter t is outside the ray interval.
            return false;
        }

        let intersection = ray.at(t);
        let planar_intersection_vector = &intersection - &self.origin;
        let planar_u = self.w.dot(&planar_intersection_vector.cross(&self.v));
        let planar_v = self.w.dot(&self.u.cross(&planar_intersection_vector));
        if planar_u < 0. || planar_u > 1. || planar_v < 0. || planar_v > 1. {
            // The hit point lies within the planar shape using its plane coordinates.
            return false;
        } else {
            record.uv = Some(Vector2::new(planar_u, planar_v));
        }

        record.t = t;
        record.point = Some(intersection);
        record.material = Some(Rc::clone(&self.material));
        record.set_face_normal(ray, &self.normal);

        return true;
    }

    fn bounding_box(&self) -> &AxisAlignedBoundingBox {
        &self.bbox
    }

    fn pdf_value(&self, origin: &Vector3, direction: &Vector3) -> f32 {
        let mut record = HitRecord::new();
        if !self.hit(
            &Ray::new(*origin, *direction),
            &Interval::new(0.001, f32::INFINITY),
            &mut record,
        ) {
            return 0.;
        }
        let HitRecord {
            normal: Some(normal),
            ..
        } = record
        else {
            return 0.;
        };

        let distance_squared = record.t * record.t * direction.length_squared();
        let cosine = f32::abs(direction.dot(&normal) / direction.length());

        distance_squared / (cosine * self.area)
    }

    fn random(&self, origin: &Point3) -> Point3 {
        let mut rng = rand::thread_rng();
        let point = self
            .origin
            .add(&self.u.mul(rng.gen::<f32>()))
            .add(&self.v.mul(rng.gen::<f32>()));
        &point - origin
    }
}
