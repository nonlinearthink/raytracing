use crate::core::{Color3, Point3, Vector2};

use super::Texture;

#[derive(Debug, Clone)]
pub struct SolidColorTexture {
    color: Color3,
}

impl SolidColorTexture {
    pub fn new(color: Color3) -> SolidColorTexture {
        SolidColorTexture { color }
    }

    pub fn new_with_floats(r: f32, g: f32, b: f32) -> SolidColorTexture {
        SolidColorTexture {
            color: Color3::new(r, g, b),
        }
    }
}

impl Texture for SolidColorTexture {
    fn value(&self, _uv: &Vector2, _point: &Point3) -> Color3 {
        self.color
    }
}
