use crate::core::{Interval, Ray, Vector3};

#[derive(Debug, Clone, Default)]
pub struct AxisAlignedBoundingBox {
    pub x: Interval,
    pub y: Interval,
    pub z: Interval,
}

impl AxisAlignedBoundingBox {
    pub fn new(x: Interval, y: Interval, z: Interval) -> Self {
        Self { x, y, z }
    }

    pub fn from_bounding_vector(min: &Vector3, max: &Vector3) -> Self {
        Self {
            x: Interval::new(min.x, max.x),
            y: Interval::new(min.y, max.y),
            z: Interval::new(min.z, max.z),
        }
    }

    pub fn axis(&self, n: usize) -> &Interval {
        match n {
            1 => &self.y,
            2 => &self.z,
            _ => &self.x,
        }
    }

    pub fn merge(&self, aabb: &Self) -> Self {
        Self {
            x: self.x.merge(&aabb.x),
            y: self.y.merge(&aabb.y),
            z: self.z.merge(&aabb.z),
        }
    }

    pub fn hit(&self, ray: &Ray, ray_interval: &mut Interval) -> bool {
        for i in 0..3 {
            let t_per_unit_length = 1. / ray.direction[i];
            let origin = ray.origin[i];
            let mut t_min = (self.axis(i).min - origin) * t_per_unit_length;
            let mut t_max = (self.axis(i).max - origin) * t_per_unit_length;

            if t_per_unit_length < 0. {
                std::mem::swap(&mut t_min, &mut t_max)
            }

            if t_min > ray_interval.min {
                ray_interval.min = t_min;
            }
            if t_max < ray_interval.max {
                ray_interval.max = t_max;
            }

            if ray_interval.max < ray_interval.min {
                return false;
            }
        }
        return true;
    }
}
