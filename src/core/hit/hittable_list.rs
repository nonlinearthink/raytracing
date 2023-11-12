use super::{HitRecord, Hittable};
use crate::core::{AxisAlignedBoundingBox, Interval, Ray};
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
}
