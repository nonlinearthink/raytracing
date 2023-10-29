use dyn_clone::{clone_trait_object, DynClone};
use std::fmt;

use super::{Color3, HitRecord, Ray, Vector3};

pub trait Material: fmt::Debug + DynClone {
    fn scatter(
        &self,
        ray_in: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut Color3,
        ray_scattered: &mut Ray,
    ) -> bool;
    fn as_dyn(&self) -> &dyn Material;
}

clone_trait_object!(Material);

#[derive(Debug, Copy, Clone)]
pub struct LambertianMaterial {
    albedo: Color3,
}

impl LambertianMaterial {
    pub fn new() -> LambertianMaterial {
        LambertianMaterial {
            albedo: Color3::zero(),
        }
    }
}

impl Material for LambertianMaterial {
    fn scatter(
        &self,
        _ray_in: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut Color3,
        ray_scattered: &mut Ray,
    ) -> bool {
        if hit_record.normal.is_some() || hit_record.point.is_some() {
            let normal = hit_record.normal.unwrap();
            let mut scatter_direction = normal + &Vector3::random_unit_vector(None);
            if scatter_direction.equals_zero() {
                scatter_direction = normal;
            }

            attenuation.clone_from(&self.albedo);
            ray_scattered.origin = hit_record.point.unwrap();
            ray_scattered.direction = scatter_direction;

            true
        } else {
            false
        }
    }

    fn as_dyn(&self) -> &dyn Material {
        self
    }
}
