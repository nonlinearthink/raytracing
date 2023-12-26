mod hit_record;
mod hittable_list;

pub use hit_record::*;
pub use hittable_list::*;

use super::{AxisAlignedBoundingBox, Interval, Ray};
use core::fmt;
use std::cmp::Ordering;

/// Interface for all hittable objects.
pub trait Hittable: fmt::Debug {
    /// Returns true if the ray hits this object, otherwise false.
    fn hit(&self, ray: &Ray, ray_interval: &Interval, record: &mut HitRecord) -> bool;

    /// Returns the bounding box of this object.
    fn bounding_box(&self) -> &AxisAlignedBoundingBox;
}

/// Returns the order of the two hittable objects.
pub fn compare_hittable_objects(
    hittable1: &dyn Hittable,
    hittable2: &dyn Hittable,
    axis_index: usize,
) -> Ordering {
    let min1 = hittable1.bounding_box().axis(axis_index).min;
    let min2 = hittable2.bounding_box().axis(axis_index).min;
    if min1 < min2 {
        Ordering::Less
    } else if min1 == min2 {
        Ordering::Equal
    } else {
        Ordering::Greater
    }
}
