use crate::{
    core::{Color3, HitRecord, Ray, ScatterRecord},
    traits::Material,
};
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
        scatter_record: &mut ScatterRecord,
    ) -> bool {
        scatter_record.attenuation = Color3::one();
        scatter_record.pdf = None;
        scatter_record.skip_pdf = true;

        let refraction_ratio = if hit_record.front_face {
            1.0 / self.ior
        } else {
            self.ior
        };

        if let HitRecord {
            normal: Some(normal),
            point: Some(point),
            ..
        } = hit_record
        {
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

            scatter_record.ray_scattered =
                Some(Ray::new_with_time(point.clone(), direction, ray_in.time));

            true
        } else {
            false
        }
    }
}
