use crate::{
    core::{Point3, Vector3},
    traits::{Hittable, ProbabilityDensityFunction},
};
use std::rc::Rc;

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
