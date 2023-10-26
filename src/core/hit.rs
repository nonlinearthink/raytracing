use super::{Interval, Point3, Ray, Vector3};

pub trait Hittable {
    fn hit(&self, ray: &Ray, ray_interval: &Interval, record: &mut HitRecord) -> bool;
}

#[derive(Debug, Copy, Clone)]
pub struct HitRecord {
    pub point: Option<Point3>,
    pub normal: Option<Vector3>,
    pub t: f32,
    front_face: bool,
}

impl HitRecord {
    pub fn new() -> HitRecord {
        HitRecord {
            point: None,
            normal: None,
            t: f32::INFINITY,
            front_face: false,
        }
    }

    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vector3) {
        // Sets the hit record normal vector.
        let unit_outward_normal = outward_normal.normolize();

        self.front_face = ray.direction.dot(outward_normal) < 0.;
        self.normal = if self.front_face {
            Some(unit_outward_normal)
        } else {
            Some(-unit_outward_normal)
        };
    }
}

pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, ray_interval: &Interval, record: &mut HitRecord) -> bool {
        record.t = ray_interval.max;

        let mut is_hitted = false;
        let mut hit_record = HitRecord::new();

        for object in self.objects.iter() {
            if object.hit(
                ray,
                &Interval::new(ray_interval.min, record.t),
                &mut hit_record,
            ) {
                is_hitted = true;
                record.clone_from(&hit_record);
            }
        }

        is_hitted
    }
}
