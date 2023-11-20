use super::Material;
use crate::core::{Color3, HitRecord, Ray, Texture, Vector2, Vector3};
use std::{
    ops::{Add, Mul},
    rc::Rc,
};

#[derive(Debug)]
pub struct MetalMaterial {
    albedo: Rc<dyn Texture>,
    fuzz: f32,
}

impl MetalMaterial {
    pub fn new(albedo: Rc<dyn Texture>, fuzz: f32) -> Self {
        Self {
            albedo,
            fuzz: f32::max(0., f32::min(fuzz, 1.)),
        }
    }
}

impl Material for MetalMaterial {
    fn scatter(
        &self,
        ray_in: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut Color3,
        ray_scattered: &mut Ray,
    ) -> bool {
        if hit_record.normal.is_some() && hit_record.point.is_some() {
            let normal = hit_record.normal.unwrap();
            let point = hit_record.point.unwrap();
            let reflected = ray_in.direction.normolize().reflect(&normal);

            ray_scattered.origin = point;
            ray_scattered.direction =
                reflected.add(&(Vector3::random_unit_vector().mul(self.fuzz)));
            ray_scattered.time = ray_in.time;
            attenuation.clone_from(&self.albedo.value(&Vector2::zero(), &point));

            ray_scattered.direction.dot(&normal) > 0.
        } else {
            false
        }
    }
}
