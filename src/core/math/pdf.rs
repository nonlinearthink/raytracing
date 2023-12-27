use super::{OrthonormalBasis, Point3, Vector3};
use crate::core::Hittable;
use std::rc::Rc;

pub trait ProbabilityDensityFunction {
    fn value(&self, direction: &Vector3) -> f32;
    fn generate(&self) -> Vector3;
}

pub struct SpherePDF {}

impl ProbabilityDensityFunction for SpherePDF {
    fn value(&self, _direction: &Vector3) -> f32 {
        1. / (4. * std::f32::consts::PI)
    }

    fn generate(&self) -> Vector3 {
        Vector3::random_unit_vector()
    }
}

pub struct CosinePDF {
    onb: OrthonormalBasis,
}

impl CosinePDF {
    pub fn new(w: Vector3) -> Self {
        Self {
            onb: OrthonormalBasis::new_with_w(&w),
        }
    }
}

impl ProbabilityDensityFunction for CosinePDF {
    fn value(&self, direction: &Vector3) -> f32 {
        let cosine_theta = direction.normolize().dot(self.onb.w());
        f32::max(0., cosine_theta / std::f32::consts::PI)
    }

    fn generate(&self) -> Vector3 {
        self.onb.local(&Vector3::random_cosine_direction())
    }
}

pub struct HittablePDF {
    objects: Rc<dyn Hittable>,
    origin: Point3,
}

impl HittablePDF {
    pub fn new(objects: Rc<dyn Hittable>, origin: Point3) -> Self {
        Self { objects, origin }
    }
}

impl ProbabilityDensityFunction for HittablePDF {
    fn value(&self, direction: &Vector3) -> f32 {
        self.objects.pdf_value(&self.origin, direction)
    }

    fn generate(&self) -> Vector3 {
        self.objects.random(&self.origin)
    }
}
