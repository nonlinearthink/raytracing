use super::Texture;
use crate::core::{Color3, Point3, Vector2};

/**
Solid Color Texture

A Texture that always returns a single color.

# Examples
```
use raytracing::core::{Color3, Point3, SolidColorTexture, Texture, Vector2};

let texture = SolidColorTexture::new(1., 0., 0.);
let color = texture.value(&Vector2::new(0., 0.), &Point3::new(0., 0., 0.));
# assert_eq!(color, Color3::new(1., 0., 0.));
```
*/
#[derive(Debug)]
pub struct SolidColorTexture {
    /// Color of the texture.
    color: Color3,
}

impl SolidColorTexture {
    /// Create a new `SolidColorTexture` with three color components, `r`, `g` and `b`, each in the range `[0, 1]`.
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Self {
            color: Color3::new(r, g, b),
        }
    }

    /// Create a new `SolidColorTexture` with the given color.
    pub fn new_with_color(color: Color3) -> Self {
        Self { color }
    }
}

impl Texture for SolidColorTexture {
    fn value(&self, _uv: &Vector2, _point: &Point3) -> Color3 {
        self.color
    }
}
