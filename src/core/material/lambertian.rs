use crate::core::{Color3, HitRecord, Ray, Vector3};

use super::Material;

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
        ray_in: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut Color3,
        ray_scattered: &mut Ray,
    ) -> bool {
        if hit_record.normal.is_some() && hit_record.point.is_some() {
            let normal = hit_record.normal.unwrap();
            let mut scatter_direction = normal + &Vector3::random_unit_vector();
            if scatter_direction.equals_zero() {
                scatter_direction = normal;
            }

            attenuation.clone_from(&self.albedo);
            ray_scattered.origin = hit_record.point.unwrap();
            ray_scattered.direction = scatter_direction;
            ray_scattered.time = ray_in.time;

            true
        } else {
            false
        }
    }
}
