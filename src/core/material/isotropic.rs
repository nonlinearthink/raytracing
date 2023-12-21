use super::Material;
use crate::core::{Color3, HitRecord, Ray, SolidColorTexture, Texture, Vector3};
use std::rc::Rc;

#[derive(Debug)]
pub struct IsotropicMaterial {
    pub albedo: Rc<dyn Texture>,
}

impl IsotropicMaterial {
    pub fn new(albedo: Rc<dyn Texture>) -> IsotropicMaterial {
        IsotropicMaterial { albedo }
    }

    pub fn new_with_color(color: Color3) -> IsotropicMaterial {
        IsotropicMaterial {
            albedo: Rc::new(SolidColorTexture::new(color)),
        }
    }
}

impl Material for IsotropicMaterial {
    fn scatter(
        &self,
        ray_in: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut Color3,
        ray_scattered: &mut Ray,
        pdf: &mut f32,
    ) -> bool {
        let point = hit_record.point.unwrap();
        let uv = hit_record.uv.unwrap();

        ray_scattered.origin = point;
        ray_scattered.direction = Vector3::random_unit_vector();
        ray_scattered.time = ray_in.time;
        attenuation.clone_from(&self.albedo.value(&uv, &point));
        *pdf = 1. / (4. * std::f32::consts::PI);

        true
    }

    fn scattering_pdf(
        &self,
        _ray_in: &Ray,
        _hit_record: &HitRecord,
        _ray_scattered: &mut Ray,
    ) -> f32 {
        1. / (4. * std::f32::consts::PI)
    }
}
