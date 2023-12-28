use crate::core::Vector3;

pub trait ProbabilityDensityFunction {
    fn value(&self, direction: &Vector3) -> f32;
    fn generate(&self) -> Vector3;
}
