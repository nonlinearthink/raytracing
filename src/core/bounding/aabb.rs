use crate::core::{Interval, Point3, Ray, Vector3};

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

    pub fn from_bounding_points(min: &Point3, max: &Point3) -> Self {
        Self {
            x: Interval::new(f32::min(min.x, max.x), f32::max(min.x, max.x)),
            y: Interval::new(f32::min(min.y, max.y), f32::max(min.y, max.y)),
            z: Interval::new(f32::min(min.z, max.z), f32::max(min.z, max.z)),
        }
    }

    pub fn axis(&self, n: usize) -> &Interval {
        match n {
            1 => &self.y,
            2 => &self.z,
            _ => &self.x,
        }
    }

    pub fn merge(&self, rhs: &Self) -> Self {
        Self {
            x: self.x.merge(&rhs.x),
            y: self.y.merge(&rhs.y),
            z: self.z.merge(&rhs.z),
        }
    }

    pub fn pad(&self) -> Self {
        let delta = 0.0001;
        let x_interval = if self.x.size() < delta {
            self.x.expand(delta)
        } else {
            self.x.clone()
        };
        let y_interval = if self.y.size() < delta {
            self.y.expand(delta)
        } else {
            self.y.clone()
        };
        let z_interval = if self.z.size() < delta {
            self.z.expand(delta)
        } else {
            self.z.clone()
        };

        Self {
            x: x_interval,
            y: y_interval,
            z: z_interval,
        }
    }

    pub fn hit(&self, ray: &Ray, ray_interval: &Interval) -> bool {
        for i in 0..3 {
            let t_per_unit_length = 1. / ray.direction[i];
            let origin = ray.origin[i];
            let mut t_min = (self.axis(i).min - origin) * t_per_unit_length;
            let mut t_max = (self.axis(i).max - origin) * t_per_unit_length;

            if t_per_unit_length < 0. {
                std::mem::swap(&mut t_min, &mut t_max)
            }

            if t_min < ray_interval.min {
                t_min = ray_interval.min;
            }
            if t_max > ray_interval.max {
                t_max = ray_interval.max;
            }

            if t_max < t_min {
                return false;
            }
        }
        return true;
    }
}

impl std::ops::Add<&Vector3> for &AxisAlignedBoundingBox {
    type Output = AxisAlignedBoundingBox;

    fn add(self, offset: &Vector3) -> Self::Output {
        AxisAlignedBoundingBox::new(&self.x + offset.x, &self.y + offset.y, &self.z + offset.z)
    }
}
