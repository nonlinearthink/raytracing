use rand::Rng;
use std::cmp::Ordering;

use super::AxisAlignedBoundingBox;
use crate::core::{HitRecord, Hittable, HittableList, Interval, Ray};

fn compare_axis(object1: &dyn Hittable, object2: &dyn Hittable, axis_index: usize) -> Ordering {
    if object1.bounding_box().axis(axis_index).min < object2.bounding_box().axis(axis_index).min {
        Ordering::Less
    } else {
        Ordering::Greater
    }
}

#[derive(Debug, Clone)]
pub struct BoundingVolumesHierarchicalNode {
    pub bbox: AxisAlignedBoundingBox,
    pub left: Option<Box<dyn Hittable>>,
    pub right: Option<Box<dyn Hittable>>,
}

impl BoundingVolumesHierarchicalNode {
    pub fn new(list: &mut HittableList) -> Self {
        let length = list.objects.len();
        Self::split(&mut list.objects, 0, length)
    }

    fn split(objects: &mut Vec<Box<dyn Hittable>>, start: usize, end: usize) -> Self {
        let left: Option<Box<dyn Hittable>>;
        let right: Option<Box<dyn Hittable>>;

        let mut rng = rand::thread_rng();
        let axis: usize = rng.gen_range(0..3);

        let objects_span = end - start;

        if objects_span == 1 {
            left = Some(Box::clone(&objects[0]));
            right = Some(Box::clone(&objects[0]));
        } else if objects_span == 2 {
            if compare_axis(&*objects[start], &*objects[start + 1], axis).is_le() {
                left = Some(Box::clone(&objects[start]));
                right = Some(Box::clone(&objects[start + 1]));
            } else {
                left = Some(Box::clone(&objects[start + 1]));
                right = Some(Box::clone(&objects[start]));
            }
        } else {
            objects.sort_by(|a, b| compare_axis(&**a, &**b, axis));

            let mid = start + objects_span / 2;
            left = Some(Box::new(BoundingVolumesHierarchicalNode::split(
                objects, start, mid,
            )));
            right = Some(Box::new(BoundingVolumesHierarchicalNode::split(
                objects, mid, end,
            )));
        }

        BoundingVolumesHierarchicalNode {
            bbox: AxisAlignedBoundingBox::merge(
                left.as_ref().unwrap().bounding_box(),
                right.as_ref().unwrap().bounding_box(),
            ),
            left,
            right,
        }
    }
}

impl Hittable for BoundingVolumesHierarchicalNode {
    fn hit(&self, ray: &Ray, ray_interval: &Interval, record: &mut HitRecord) -> bool {
        if !self.bbox.hit(ray, &mut ray_interval.clone()) {
            return false;
        }

        let hit_left = match &self.left {
            Some(node) => node.hit(ray, ray_interval, record),
            None => false,
        };
        let hit_right = match &self.right {
            Some(node) => node.hit(ray, ray_interval, record),
            None => false,
        };

        hit_left || hit_right
    }

    fn bounding_box(&self) -> &AxisAlignedBoundingBox {
        &self.bbox
    }
}
