use super::Material;
use crate::core::{Color3, HitRecord, Ray, SolidColorTexture, Texture, Vector3};
use std::rc::Rc;

#[derive(Debug)]
pub struct LambertianMaterial {
    albedo: Rc<dyn Texture>,
}

impl LambertianMaterial {
    pub fn new(albedo: Rc<dyn Texture>) -> Self {
        Self { albedo }
    }

    pub fn new_with_color(color: Color3) -> Self {
        Self {
            albedo: Rc::new(SolidColorTexture::new(color)),
        }
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
        if hit_record.normal.is_some() && hit_record.point.is_some() && hit_record.uv.is_some() {
            let normal = hit_record.normal.unwrap();
            let point = hit_record.point.unwrap();
            let uv = hit_record.uv.unwrap();
            let mut scatter_direction = Vector3::random_on_hemisphere(&normal);
            if scatter_direction.equals_zero() {
                scatter_direction = normal;
            }

            attenuation.clone_from(&self.albedo.value(&uv, &point));
            ray_scattered.origin = point;
            ray_scattered.direction = scatter_direction;
            ray_scattered.time = ray_in.time;

            true
        } else {
            false
        }
    }

    fn scattering_pdf(
        &self,
        _ray_in: &Ray,
        _hit_record: &HitRecord,
        _ray_scattered: &mut Ray,
    ) -> f32 {
        1. / (2. * std::f32::consts::PI)
    }
}
