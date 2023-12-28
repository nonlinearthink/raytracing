use crate::{
    core::{Color3, Ray},
    traits::ProbabilityDensityFunction,
};
use std::rc::Rc;

pub struct ScatterRecord {
    pub attenuation: Color3,
    pub ray_scattered: Option<Ray>,
    pub pdf: Option<Rc<dyn ProbabilityDensityFunction>>,
    pub skip_pdf: bool,
}

impl ScatterRecord {
    pub fn new() -> Self {
        Self {
            attenuation: Color3::zero(),
            ray_scattered: None,
            pdf: None,
            skip_pdf: false,
        }
    }
}
