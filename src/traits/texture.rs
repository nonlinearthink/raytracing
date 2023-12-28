use crate::core::{Color3, Point3, Vector2};
use std::fmt;

/// Texture trait.
pub trait Texture: fmt::Debug {
    /// Return the color of the texture by uv coordinates and hit point.
    fn value(&self, uv: &Vector2, point: &Point3) -> Color3;
}
