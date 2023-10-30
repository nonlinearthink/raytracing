use dyn_clone::{clone_trait_object, DynClone};
use rand::Rng;
use std::{fmt, ops::Neg};

use super::{Color3, HitRecord, Ray, Vector3};

pub trait Material: fmt::Debug + DynClone {
    fn scatter(
        &self,
        ray_in: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut Color3,
        ray_scattered: &mut Ray,
    ) -> bool;
}

clone_trait_object!(Material);

#[derive(Debug, Copy, Clone)]
pub struct LambertianMaterial {
    pub albedo: Color3,
}

impl LambertianMaterial {
    pub fn new(albedo_optional: Option<Color3>) -> LambertianMaterial {
        let albedo = match albedo_optional {
            Some(color) => color,
            None => Color3::zero(),
        };
        LambertianMaterial { albedo }
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
        if hit_record.normal.is_some() && hit_record.point.is_some() {
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
}

#[derive(Debug, Copy, Clone)]
pub struct MetalMaterial {
    pub albedo: Color3,
    pub fuzz: f32,
}

impl MetalMaterial {
    pub fn new(albedo_optional: Option<Color3>, fuzz: f32) -> MetalMaterial {
        let albedo = match albedo_optional {
            Some(color) => color,
            None => Color3::zero(),
        };
        MetalMaterial {
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
            let reflected = ray_in.direction.normolize().reflect(&normal);

            ray_scattered.origin = hit_record.point.unwrap();
            let fuzz_vector = Vector3::random_unit_vector(None);
            ray_scattered.direction = reflected + &(fuzz_vector * self.fuzz);
            attenuation.clone_from(&self.albedo);

            ray_scattered.direction.dot(&normal) > 0.
        } else {
            false
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct DielectricMaterial {
    pub ior: f32, // Index of Refraction
}

impl DielectricMaterial {
    pub fn new(ior: f32) -> DielectricMaterial {
        DielectricMaterial { ior }
    }

    fn reflectance(cosine: f32, ref_idx: f32) -> f32 {
        // Use Schlick's approximation for reflectance.
        let mut r0 = (1. - ref_idx) / (1. + ref_idx);
        r0 = r0 * r0;
        r0 + (1. - r0) * f32::powf(1. - cosine, 5.)
    }
}

impl Material for DielectricMaterial {
    fn scatter(
        &self,
        ray_in: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut Color3,
        ray_scattered: &mut Ray,
    ) -> bool {
        attenuation.clone_from(&Color3::one());
        let refraction_ratio = if hit_record.front_face {
            1.0 / self.ior
        } else {
            self.ior
        };

        if hit_record.normal.is_some() && hit_record.point.is_some() {
            let normal = hit_record.normal.unwrap();
            let unit_direction = ray_in.direction.normolize();
            let cos_theta = f32::min(unit_direction.neg().dot(&normal), 1.);
            let sin_theta = f32::sqrt(1. - cos_theta * cos_theta);

            let cannot_refract = refraction_ratio * sin_theta > 1.;
            let direction = if cannot_refract {
                unit_direction.reflect(&normal)
            } else {
                unit_direction.refract(&normal, refraction_ratio)
            };

            ray_scattered.origin = hit_record.point.unwrap();
            ray_scattered.direction = direction;

            true
        } else {
            false
        }
    }
}
