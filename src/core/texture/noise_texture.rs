use std::ops::Mul;

use super::Texture;
use crate::core::{Color3, Perlin, Point3, Vector2};

#[derive(Debug)]
pub struct NoiseTexture {
    noise: Perlin,
    scale: f32,
    marble_effect: bool,
}

impl NoiseTexture {
    pub fn new(scale: f32) -> Self {
        NoiseTexture {
            noise: Perlin::new(),
            scale,
            marble_effect: false,
        }
    }

    pub fn new_with_marble_effect(scale: f32) -> Self {
        NoiseTexture {
            noise: Perlin::new(),
            scale,
            marble_effect: true,
        }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _uv: &Vector2, point: &Point3) -> Color3 {
        let scaled_point = point * self.scale;
        if self.marble_effect {
            Color3::new(1., 1., 1.)
                .mul(0.5)
                .mul(1. + f32::sin(scaled_point.z + 10. * self.noise.turbulence(&scaled_point, 7)))
        } else {
            &Color3::new(1., 1., 1.) * self.noise.turbulence(&scaled_point, 7)
        }
    }
}
