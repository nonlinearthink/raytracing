use crate::{core::Vector3, traits::ProbabilityDensityFunction};

pub struct SpherePDF {}

impl SpherePDF {
    pub fn new() -> Self {
        Self {}
    }
}

impl ProbabilityDensityFunction for SpherePDF {
    fn value(&self, _direction: &Vector3) -> f32 {
        1. / (4. * std::f32::consts::PI)
    }

    fn generate(&self) -> Vector3 {
        Vector3::random_unit_vector()
    }
}
