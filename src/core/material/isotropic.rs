use std::rc::Rc;

use crate::core::{Color3, SolidColorTexture, Texture, Vector3};

use super::Material;

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
        ray_in: &crate::core::Ray,
        hit_record: &crate::core::HitRecord,
        attenuation: &mut Color3,
        ray_scattered: &mut crate::core::Ray,
    ) -> bool {
        let point = hit_record.point.unwrap();
        let uv = hit_record.uv.unwrap();

        ray_scattered.origin = point;
        ray_scattered.direction = Vector3::random_unit_vector();
        ray_scattered.time = ray_in.time;
        attenuation.clone_from(&self.albedo.value(&uv, &point));

        true
    }
}
