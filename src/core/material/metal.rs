use crate::core::{Color3, HitRecord, Ray, Vector3};

use super::Material;

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
            let fuzz_vector = Vector3::random_unit_vector();
            ray_scattered.direction = reflected + &(fuzz_vector * self.fuzz);
            attenuation.clone_from(&self.albedo);

            ray_scattered.direction.dot(&normal) > 0.
        } else {
            false
        }
    }
}
