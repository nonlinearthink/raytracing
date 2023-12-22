use super::Texture;
use crate::core::{Color3, Point3, Vector2};

/**
Solid color texture.

# Examples
```
use raytracing::core::{Color3, Point3, SolidColorTexture, Texture, Vector2};

let texture = SolidColorTexture::new(Color3::new(1., 0., 0.));
# assert_eq!(texture.value(&Vector2::new(0., 0.), &Point3::new(0., 0., 0.)), Color3::new(1., 0., 0.));
```
 */
#[derive(Debug)]
pub struct SolidColorTexture {
    /// Color of the texture.
    color: Color3,
}

impl SolidColorTexture {
    /// Create a new `SolidColorTexture` with the given color.
    pub fn new(color: Color3) -> Self {
        Self { color }
    }

    /// Create a new `SolidColorTexture` with the given rgb.
    pub fn new_with_floats(r: f32, g: f32, b: f32) -> Self {
        Self {
            color: Color3::new(r, g, b),
        }
    }
}

impl Texture for SolidColorTexture {
    fn value(&self, _uv: &Vector2, _point: &Point3) -> Color3 {
        self.color
    }
}
