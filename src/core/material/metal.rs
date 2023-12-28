use crate::{
    core::{HitRecord, Ray, ScatterRecord, Vector2, Vector3},
    traits::{Material, Texture},
};
use std::{
    ops::{Add, Mul},
    rc::Rc,
};

#[derive(Debug)]
pub struct MetalMaterial {
    albedo: Rc<dyn Texture>,
    fuzz: f32,
}

impl MetalMaterial {
    pub fn new(albedo: Rc<dyn Texture>, fuzz: f32) -> Self {
        Self {
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
        scatter_record: &mut ScatterRecord,
    ) -> bool {
        if let HitRecord {
            point: Some(point),
            normal: Some(normal),
            ..
        } = hit_record
        {
            let reflected = ray_in.direction.normolize().reflect(&normal);
            let ray_scattered = Ray::new_with_time(
                point.clone(),
                reflected.add(&(Vector3::random_unit_vector().mul(self.fuzz))),
                ray_in.time,
            );

            let is_hitted = ray_scattered.direction.dot(&normal) > 0.;

            scatter_record.ray_scattered = Some(ray_scattered);
            scatter_record.attenuation = self.albedo.value(&Vector2::zero(), &point);
            scatter_record.pdf = None;
            scatter_record.skip_pdf = true;

            // TODO: test if return true directly
            is_hitted
        } else {
            false
        }
    }
}
