use crate::{
    core::{Color3, HitRecord, Ray, ScatterRecord, SolidColorTexture, SpherePDF},
    traits::{Material, Texture},
};
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
            albedo: Rc::new(SolidColorTexture::new_with_color(color)),
        }
    }
}

impl Material for IsotropicMaterial {
    fn scatter(
        &self,
        _ray_in: &Ray,
        hit_record: &HitRecord,
        scatter_record: &mut ScatterRecord,
    ) -> bool {
        if let HitRecord {
            point: Some(point),
            uv: Some(uv),
            ..
        } = hit_record
        {
            scatter_record.attenuation = self.albedo.value(uv, point);
            scatter_record.pdf = Some(Rc::new(SpherePDF::new()));
            scatter_record.skip_pdf = false;

            true
        } else {
            false
        }
    }

    fn scattering_pdf(&self, _ray_in: &Ray, _hit_record: &HitRecord, _ray_scattered: &Ray) -> f32 {
        1. / (4. * std::f32::consts::PI)
    }
}
