use crate::{core::Vector3, traits::ProbabilityDensityFunction};
use std::rc::Rc;

pub struct MixturePDF {
    pdfs: [Rc<dyn ProbabilityDensityFunction>; 2],
}

impl MixturePDF {
    pub fn new(
        pdf1: Rc<dyn ProbabilityDensityFunction>,
        pdf2: Rc<dyn ProbabilityDensityFunction>,
    ) -> Self {
        Self { pdfs: [pdf1, pdf2] }
    }
}

impl ProbabilityDensityFunction for MixturePDF {
    fn value(&self, direction: &Vector3) -> f32 {
        self.pdfs[0].value(direction) * 0.5 + self.pdfs[1].value(direction) * 0.5
    }

    fn generate(&self) -> Vector3 {
        if rand::random::<f32>() < 0.5 {
            self.pdfs[0].generate()
        } else {
            self.pdfs[1].generate()
        }
    }
}
