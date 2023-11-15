use super::Material;
use crate::core::{Color3, Point3, SolidColorTexture, Texture, Vector2};
use std::rc::Rc;

#[derive(Debug)]
pub struct EmissiveMaterial {
    emit: Rc<dyn Texture>,
}

impl EmissiveMaterial {
    pub fn new(texture: Rc<dyn Texture>) -> Self {
        Self { emit: texture }
    }

    pub fn new_with_color(color: Color3) -> Self {
        Self {
            emit: Rc::new(SolidColorTexture::new(color)),
        }
    }
}

impl Material for EmissiveMaterial {
    fn scatter(
        &self,
        _ray_in: &crate::core::Ray,
        _hit_record: &crate::core::HitRecord,
        _attenuation: &mut Color3,
        _ray_scattered: &mut crate::core::Ray,
    ) -> bool {
        return false;
    }

    fn emitted(&self, uv: &Vector2, point: &Point3) -> Color3 {
        self.emit.value(uv, point)
    }
}
