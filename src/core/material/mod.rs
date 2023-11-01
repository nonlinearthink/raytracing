use dyn_clone::{clone_trait_object, DynClone};
use std::fmt;

use super::{Color3, HitRecord, Ray};

mod dielectric;
mod lambertian;
mod metal;

pub use dielectric::DielectricMaterial;
pub use lambertian::LambertianMaterial;
pub use metal::MetalMaterial;

pub trait Material: fmt::Debug + DynClone {
    fn scatter(
        &self,
        ray_in: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut Color3,
        ray_scattered: &mut Ray,
    ) -> bool;
}

clone_trait_object!(Material);
