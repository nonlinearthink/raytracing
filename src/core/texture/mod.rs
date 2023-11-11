mod checker_texture;
mod solid_texture;

pub use checker_texture::*;
pub use solid_texture::*;

use dyn_clone::{DynClone, clone_trait_object};
use std::fmt;

use super::{Color3, Point3, Vector2};

pub trait Texture: fmt::Debug + DynClone {
    fn value(&self, uv: &Vector2, point: &Point3) -> Color3;
}

clone_trait_object!(Texture);
