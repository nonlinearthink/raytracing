use std::ops::{Add, Mul};

use super::vector3::Vector3;

pub struct OrthonormalBasis {
    axis: [Vector3; 3],
}

impl OrthonormalBasis {
    pub fn new(u: Vector3, v: Vector3, w: Vector3) -> Self {
        OrthonormalBasis { axis: [u, v, w] }
    }

    pub fn new_with_w(w: &Vector3) -> Self {
        let unit_w = w.normolize();
        let aux = if f32::abs(unit_w.x) > 0.9 {
            Vector3::new(0., 1., 0.)
        } else {
            Vector3::new(1., 0., 0.)
        };
        let v = unit_w.cross(&aux).normolize();
        let u = unit_w.cross(&v);
        OrthonormalBasis {
            axis: [u, v, unit_w],
        }
    }

    pub fn u(&self) -> &Vector3 {
        &self.axis[0]
    }

    pub fn v(&self) -> &Vector3 {
        &self.axis[1]
    }

    pub fn w(&self) -> &Vector3 {
        &self.axis[2]
    }

    pub fn local(&self, vec: &Vector3) -> Vector3 {
        self.axis[0]
            .mul(vec.x)
            .add(&self.axis[1].mul(vec.y))
            .add(&self.axis[2].mul(vec.z))
    }

    pub fn local_with_floats(&self, u: f32, v: f32, w: f32) -> Vector3 {
        self.axis[0]
            .mul(u)
            .add(&self.axis[1].mul(v))
            .add(&self.axis[2].mul(w))
    }
}
