use crate::{
    core::{AxisAlignedBoundingBox, HitRecord, Interval, Point3, Ray, Vector3},
    traits::Hittable,
};
use rand::Rng;
use std::rc::Rc;

#[derive(Debug)]
pub struct HittableList {
    pub objects: Vec<Rc<dyn Hittable>>,
    bbox: AxisAlignedBoundingBox,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            bbox: AxisAlignedBoundingBox {
                x: Interval::empty(),
                y: Interval::empty(),
                z: Interval::empty(),
            },
        }
    }

    pub fn add(&mut self, object: Rc<dyn Hittable>) {
        self.bbox = self.bbox.merge(&object.bounding_box());
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
                &mut Interval::new(ray_interval.min, record.t),
                &mut hit_record,
            ) {
                is_hitted = true;
                record.clone_from(&hit_record);
            }
        }

        is_hitted
    }

    fn bounding_box(&self) -> &AxisAlignedBoundingBox {
        &self.bbox
    }

    fn pdf_value(&self, origin: &Point3, direction: &Vector3) -> f32 {
        let weight = 1.0 / self.objects.len() as f32;

        let mut sum = 0.0;
        for object in &self.objects {
            sum += weight * object.pdf_value(origin, direction);
        }

        return sum;
    }

    fn random(&self, origin: &Point3) -> Vector3 {
        let mut rng = rand::thread_rng();

        let size = self.objects.len();
        let random_index = rng.gen_range(0..size);

        self.objects[random_index].random(origin)
    }
}
