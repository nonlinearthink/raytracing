use crate::{
    core::{OrthonormalBasis, Vector3},
    traits::ProbabilityDensityFunction,
};

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
