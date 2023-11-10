mod hit_record;
mod hittable_list;

pub use hit_record::*;
pub use hittable_list::*;

use core::fmt;
use dyn_clone::{clone_trait_object, DynClone};
use std::cmp::Ordering;

use super::{AxisAlignedBoundingBox, Interval, Ray};

pub trait Hittable: fmt::Debug + DynClone {
    fn hit(&self, ray: &Ray, ray_interval: &Interval, record: &mut HitRecord) -> bool;

    fn bounding_box(&self) -> &AxisAlignedBoundingBox;
}

clone_trait_object!(Hittable);

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