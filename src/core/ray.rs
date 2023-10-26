use std::ops::{Add, Mul};

use super::{Point3, Vector3};

#[derive(Debug)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vector3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vector3) -> Ray {
        Ray { origin, direction }
    }

    pub fn at(&self, t: f32) -> Point3 {
        self.direction.mul(t).add(&self.origin)
    }
}
