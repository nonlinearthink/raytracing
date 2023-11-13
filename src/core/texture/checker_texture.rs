use super::{SolidColorTexture, Texture};
use crate::core::{Color3, Point3};
use std::rc::Rc;

#[derive(Debug)]
pub struct CheckerTexture {
    pub even: Rc<dyn Texture>,
    pub odd: Rc<dyn Texture>,
    invert_scale: f32,
}

impl CheckerTexture {
    pub fn new(scale: f32, even: Rc<dyn Texture>, odd: Rc<dyn Texture>) -> Self {
        Self {
            even,
            odd,
            invert_scale: 1. / scale,
        }
    }

    pub fn new_with_solid_color(scale: f32, even_color: Color3, odd_color: Color3) -> Self {
        Self {
            even: Rc::new(SolidColorTexture::new(even_color)),
            odd: Rc::new(SolidColorTexture::new(odd_color)),
            invert_scale: 1. / scale,
        }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, uv: &crate::core::Vector2, point: &Point3) -> Color3 {
        let x_factor = f32::floor(self.invert_scale * point.x) as i32;
        let y_fatcor = f32::floor(self.invert_scale * point.y) as i32;
        let z_factor = f32::floor(self.invert_scale * point.z) as i32;

        let is_even = (x_factor + y_fatcor + z_factor) % 2 == 0;

        if is_even {
            self.even.value(uv, point)
        } else {
            self.odd.value(uv, point)
        }
    }
}
