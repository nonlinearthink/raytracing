use crate::core::{Color3, HitRecord, Ray, ScatterRecord, Vector2, Vector3};
use std::fmt;

pub trait Material: fmt::Debug {
    fn scatter(
        &self,
        ray_in: &Ray,
        hit_record: &HitRecord,
        scatter_record: &mut ScatterRecord,
    ) -> bool;

    fn scattering_pdf(
        &self,
        _ray_in: &Ray,
        _hit_record: &HitRecord,
        _ray_scattered: &mut Ray,
    ) -> f32 {
        return 0.;
    }

    fn emitted(
        &self,
        _ray_in: &Ray,
        _hit_record: &HitRecord,
        _uv: &Vector2,
        _point: &Vector3,
    ) -> Color3 {
        Color3::zero()
    }
}
