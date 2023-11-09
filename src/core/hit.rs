use core::fmt;
use dyn_clone::{clone_trait_object, DynClone};
use std::cmp::Ordering;

use super::{material::Material, AxisAlignedBoundingBox, Interval, Point3, Ray, Vector3};

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

#[derive(Debug, Clone)]
pub struct HitRecord {
    pub material: Option<Box<dyn Material>>,
    pub point: Option<Point3>,
    pub normal: Option<Vector3>,
    pub t: f32,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new() -> HitRecord {
        HitRecord {
            material: None,
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

#[derive(Debug, Clone)]
pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
    aabb: AxisAlignedBoundingBox,
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList {
            objects: Vec::new(),
            aabb: AxisAlignedBoundingBox {
                x: Interval::empty(),
                y: Interval::empty(),
                z: Interval::empty(),
            },
        }
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.aabb = self.aabb.merge(&object.bounding_box());
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
        &self.aabb
    }
}
