use std::ops::{Add, Mul};

use super::{Point3, Vector3};

#[derive(Debug, Copy, Clone)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vector3,
    pub time: f32,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vector3) -> Self {
        Self {
            origin,
            direction,
            time: 0.,
        }
    }

    pub fn new_with_time(origin: Point3, direction: Vector3, time: f32) -> Self {
        Self {
            origin,
            direction,
            time,
        }
    }

    pub fn at(&self, t: f32) -> Point3 {
        self.direction.mul(t).add(&self.origin)
    }
}
