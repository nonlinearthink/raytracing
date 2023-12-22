use super::Vector3;

pub trait ProbabilityDensityFunction {
    fn value(direction: &Vector3) -> f32;
    fn generate() -> Vector3;
}
