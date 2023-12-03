use super::AxisAlignedBoundingBox;
use crate::core::{compare_hittable_objects, HitRecord, Hittable, HittableList, Interval, Ray};
use rand::Rng;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct BVHNode {
    pub bbox: AxisAlignedBoundingBox,
    pub left: Option<Rc<dyn Hittable>>,
    pub right: Option<Rc<dyn Hittable>>,
}

impl BVHNode {
    pub fn new(list: &mut HittableList) -> Self {
        let length = list.objects.len();
        Self::split(&mut list.objects, 0, length)
    }

    fn split(objects: &mut Vec<Rc<dyn Hittable>>, start: usize, end: usize) -> Self {
        let left: Option<Rc<dyn Hittable>>;
        let right: Option<Rc<dyn Hittable>>;

        let mut rng = rand::thread_rng();
        let axis: usize = rng.gen_range(0..3);

        let objects_span = end - start;

        if objects_span == 1 {
            left = Some(Rc::clone(&objects[start]));
            right = Some(Rc::clone(&objects[start]));
        } else if objects_span == 2 {
            if compare_hittable_objects(&*objects[start], &*objects[start + 1], axis).is_le() {
                left = Some(Rc::clone(&objects[start]));
                right = Some(Rc::clone(&objects[start + 1]));
            } else {
                left = Some(Rc::clone(&objects[start + 1]));
                right = Some(Rc::clone(&objects[start]));
            }
        } else {
            objects[start..end].sort_by(|a, b| compare_hittable_objects(&**a, &**b, axis));

            let mid = start + objects_span / 2;
            left = Some(Rc::new(Self::split(objects, start, mid)));
            right = Some(Rc::new(Self::split(objects, mid, end)));
        }

        Self {
            bbox: AxisAlignedBoundingBox::merge(
                left.as_ref().unwrap().bounding_box(),
                right.as_ref().unwrap().bounding_box(),
            ),
            left,
            right,
        }
    }
}

impl Hittable for BVHNode {
    fn hit(&self, ray: &Ray, ray_interval: &Interval, record: &mut HitRecord) -> bool {
        if !self.bbox.hit(ray, ray_interval) {
            return false;
        }

        let hit_left = match &self.left {
            Some(node) => node.hit(ray, ray_interval, record),
            None => false,
        };
        let hit_right = match &self.right {
            Some(node) => node.hit(
                ray,
                &Interval::new(
                    ray_interval.min,
                    if hit_left { record.t } else { ray_interval.max },
                ),
                record,
            ),
            None => false,
        };

        hit_left || hit_right
    }

    fn bounding_box(&self) -> &AxisAlignedBoundingBox {
        &self.bbox
    }
}
