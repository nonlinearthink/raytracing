use crate::core::{Color3, HitRecord, Ray, ScatterRecord, Vector2, Vector3};
use std::fmt;

/// Material trait.
pub trait Material: fmt::Debug {
    /// Returns true if the material scatter the ray, otherwise false.
    fn scatter(
        &self,
        ray_in: &Ray,
        hit_record: &HitRecord,
        scatter_record: &mut ScatterRecord,
    ) -> bool;

    /// Returns the color of light emitted.
    fn emitted(
        &self,
        _ray_in: &Ray,
        _hit_record: &HitRecord,
        _uv: &Vector2,
        _point: &Vector3,
    ) -> Color3 {
        Color3::zero()
    }

    fn scattering_pdf(&self, _ray_in: &Ray, _hit_record: &HitRecord, _ray_scattered: &Ray) -> f32 {
        return 0.;
    }
}
