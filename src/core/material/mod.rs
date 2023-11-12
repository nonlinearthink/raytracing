mod dielectric;
mod lambertian;
mod metal;

pub use dielectric::*;
pub use lambertian::*;
pub use metal::*;

use super::{Color3, HitRecord, Ray};
use std::fmt;

pub trait Material: fmt::Debug {
    fn scatter(
        &self,
        ray_in: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut Color3,
        ray_scattered: &mut Ray,
    ) -> bool;
}
