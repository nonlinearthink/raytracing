use super::Material;
use crate::core::{Color3, HitRecord, Ray};
use std::ops::Neg;

#[derive(Debug)]
pub struct DielectricMaterial {
    pub ior: f32, // Index of Refraction
}

impl DielectricMaterial {
    pub fn new(ior: f32) -> Self {
        Self { ior }
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
        _pdf: &mut f32,
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
            let direction = if cannot_refract
                || DielectricMaterial::reflectance(cos_theta, refraction_ratio)
                    > rand::random::<f32>()
            {
                unit_direction.reflect(&normal)
            } else {
                unit_direction.refract(&normal, refraction_ratio)
            };

            ray_scattered.origin = hit_record.point.unwrap();
            ray_scattered.direction = direction;
            ray_scattered.time = ray_in.time;

            true
        } else {
            false
        }
    }
}
