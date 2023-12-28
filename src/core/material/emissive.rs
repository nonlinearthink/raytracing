use crate::{
    core::{Color3, HitRecord, Point3, Ray, ScatterRecord, SolidColorTexture, Vector2},
    traits::{Material, Texture},
};
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
            emit: Rc::new(SolidColorTexture::new_with_color(color)),
        }
    }
}

impl Material for EmissiveMaterial {
    fn scatter(
        &self,
        _ray_in: &Ray,
        _hit_record: &HitRecord,
        _scatter_record: &mut ScatterRecord,
    ) -> bool {
        return false;
    }

    fn emitted(
        &self,
        _ray_in: &Ray,
        hit_record: &HitRecord,
        uv: &Vector2,
        point: &Point3,
    ) -> Color3 {
        if !hit_record.front_face {
            Color3::zero()
        } else {
            self.emit.value(uv, point)
        }
    }
}
