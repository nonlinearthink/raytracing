use crate::core::{AxisAlignedBoundingBox, Hittable, Material, Point3, Vector2, Vector3};
use std::rc::Rc;

#[derive(Debug)]
pub struct Quadrilateral {
    origin: Point3,
    u: Vector3,
    v: Vector3,
    material: Rc<dyn Material>,
    bbox: AxisAlignedBoundingBox,
    normal: Vector3,
    d: f32,
    w: Vector3,
}

impl Quadrilateral {
    pub fn new(origin: Point3, u: Vector3, v: Vector3, material: Rc<dyn Material>) -> Self {
        let normal = u.cross(&v).normolize();
        let d = normal.dot(&origin);
        let w = normal / normal.dot(&normal);
        Self {
            origin,
            u,
            v,
            material,
            bbox: AxisAlignedBoundingBox::from_bounding_vector(&origin, &(origin + &u + &v)).pad(),
            normal,
            d,
            w,
        }
    }
}

impl Hittable for Quadrilateral {
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
        let planar_intersection_vector = intersection - &self.origin;
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
}
