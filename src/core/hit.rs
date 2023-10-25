use super::{Point3, Ray, Vector3};

pub struct HitRecord {
    pub point: Point3,
    pub normal: Vector3,
    pub t: f32,
    pub front_face: bool,
}

impl HitRecord {
    fn set_face_normal(mut self, ray: &Ray, outward_normal: &Vector3) {
        // Sets the hit record normal vector.
        let unit_outward_normal = outward_normal.normolize();

        self.front_face = ray.direction.dot(outward_normal) < 0.;
        self.normal = if self.front_face {
            unit_outward_normal
        } else {
            -unit_outward_normal
        };
    }
}

pub trait Hittable {
    fn hit(self, ray: &Ray, min_t: f32, max_t: f32, record: &mut HitRecord) -> bool;
}
