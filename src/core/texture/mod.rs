mod checker_texture;
mod image_texture;
mod noise_texture;
mod solid_texture;

pub use checker_texture::*;
pub use image_texture::*;
pub use noise_texture::*;
pub use solid_texture::*;

use super::{Color3, Point3, Vector2};
use std::fmt;

pub trait Texture: fmt::Debug {
    fn value(&self, uv: &Vector2, point: &Point3) -> Color3;
}
