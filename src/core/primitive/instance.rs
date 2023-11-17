use crate::{
    core::{AxisAlignedBoundingBox, HitRecord, Hittable, Interval, Point3, Ray, Vector3},
    utils::deg_to_rad,
};
use std::{ops::Add, rc::Rc};

#[derive(Debug)]
pub struct TranslateInstance {
    object: Rc<dyn Hittable>,
    offset: Vector3,
    bbox: AxisAlignedBoundingBox,
}

impl TranslateInstance {
    pub fn new(object: Rc<dyn Hittable>, offset: Vector3) -> Self {
        TranslateInstance {
            object: object.clone(),
            offset,
            bbox: object.bounding_box().add(&offset),
        }
    }
}

impl Hittable for TranslateInstance {
    fn hit(&self, ray: &Ray, ray_interval: &Interval, record: &mut HitRecord) -> bool {
        let offset_ray = Ray::new_with_time(ray.origin - &self.offset, ray.direction, ray.time);

        if !self.object.hit(&offset_ray, ray_interval, record) {
            return false;
        }

        record.point = Some(record.point.unwrap() + &self.offset);

        true
    }

    fn bounding_box(&self) -> &AxisAlignedBoundingBox {
        &self.bbox
    }
}

#[derive(Debug)]
pub struct RotateYInstance {
    object: Rc<dyn Hittable>,
    sin_theta: f32,
    cos_theta: f32,
    bbox: AxisAlignedBoundingBox,
}

impl RotateYInstance {
    pub fn new(object: Rc<dyn Hittable>, angle: f32) -> Self {
        let radians = deg_to_rad(angle);
        let sin_theta = f32::sin(radians);
        let cos_theta = f32::cos(radians);

        let bbox = object.bounding_box();

        let mut min = Vector3::new(f32::INFINITY, f32::INFINITY, f32::INFINITY);
        let mut max = Vector3::new(f32::NEG_INFINITY, f32::NEG_INFINITY, f32::NEG_INFINITY);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = bbox.x.max * i as f32 + bbox.x.min * (1. - i as f32);
                    let y = bbox.y.max * j as f32 + bbox.y.min * (1. - j as f32);
                    let z = bbox.z.max * k as f32 + bbox.z.min * (1. - k as f32);

                    let new_x = cos_theta * x + sin_theta * z;
                    let new_z = -sin_theta * x + cos_theta * z;

                    let new_point = Point3::new(new_x, y, new_z);

                    min.x = f32::min(min.x, new_point.x);
                    min.y = f32::min(min.y, new_point.y);
                    min.z = f32::min(min.z, new_point.z);
                    max.x = f32::max(max.x, new_point.x);
                    max.y = f32::max(max.y, new_point.y);
                    max.z = f32::max(max.z, new_point.z);
                }
            }
        }

        return Self {
            object: object.clone(),
            sin_theta,
            cos_theta,
            bbox: AxisAlignedBoundingBox::from_bounding_points(&min, &max),
        };
    }
}
