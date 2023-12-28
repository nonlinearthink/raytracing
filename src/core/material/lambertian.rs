use crate::{
    core::{Color3, CosinePDF, HitRecord, Ray, ScatterRecord, SolidColorTexture},
    traits::{Material, Texture},
};
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
            albedo: Rc::new(SolidColorTexture::new_with_color(color)),
        }
    }
}

impl Material for LambertianMaterial {
    fn scatter(
        &self,
        _ray_in: &Ray,
        hit_record: &HitRecord,
        scatter_record: &mut ScatterRecord,
    ) -> bool {
        if let HitRecord {
            normal: Some(normal),
            point: Some(point),
            uv: Some(uv),
            ..
        } = hit_record
        {
            scatter_record.attenuation = self.albedo.value(uv, point);
            scatter_record.pdf = Some(Rc::new(CosinePDF::new(*normal)));
            scatter_record.skip_pdf = false;

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
