use crate::core::Vector3;

/// Probability density function trait.
pub trait ProbabilityDensityFunction {
    /// Return the value of the probability density function at the given direction.
    fn value(&self, direction: &Vector3) -> f32;

    /// Return a random direction from the probability density function.
    fn generate(&self) -> Vector3;
}
