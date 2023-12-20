use crate::core::{Interval, Ray, Vector3};

/// Axis-aligned bounding box
#[derive(Debug, Default)]
pub struct AxisAlignedBoundingBox {
    /// x coordinate interval
    pub x: Interval,
    /// y coordinate interval
    pub y: Interval,
    /// z coordinate interval
    pub z: Interval,
}

impl AxisAlignedBoundingBox {
    /// Create a new `AxisAlignedBoundingBox` with x, y, and z axis intervals.
    pub fn new(x: Interval, y: Interval, z: Interval) -> Self {
        Self { x, y, z }
    }

    /// Create a new `AxisAlignedBoundingBox` with min and max bounding vector.
    pub fn from_bounding_points(min: &Vector3, max: &Vector3) -> Self {
        Self {
            x: Interval::new(f32::min(min.x, max.x), f32::max(min.x, max.x)),
            y: Interval::new(f32::min(min.y, max.y), f32::max(min.y, max.y)),
            z: Interval::new(f32::min(min.z, max.z), f32::max(min.z, max.z)),
        }
    }

    /**
    Get the axis of the bounding box with the given index.
    1: y axis, 2: z axis, otherwise: x axis.

    # Examples

    ```
    use raytracing::core::{AxisAlignedBoundingBox, Interval};

    let bbox = AxisAlignedBoundingBox::new(Interval::new(0.0, 1.0), Interval::new(0.0, 2.0), Interval::new(0.0, 3.0));

    let x_axis = bbox.axis(0);
    let y_axis = bbox.axis(1);
    let z_axis = bbox.axis(2);
    let unknown_axis = bbox.axis(3);

    # assert_eq!(x_axis, &Interval::new(0.0, 1.0));
    # assert_eq!(y_axis, &Interval::new(0.0, 2.0));
    # assert_eq!(z_axis, &Interval::new(0.0, 3.0));
    # assert_eq!(unknown_axis, x_axis);
    ```
     */
    pub fn axis(&self, n: usize) -> &Interval {
        match n {
            1 => &self.y,
            2 => &self.z,
            _ => &self.x,
        }
    }

    /// Merge two axis-aligned bounding boxes.
    pub fn merge(&self, rhs: &Self) -> Self {
        Self {
            x: self.x.merge(&rhs.x),
            y: self.y.merge(&rhs.y),
            z: self.z.merge(&rhs.z),
        }
    }

    /// Pad the bounding box by a small amount called delta, make it possible for quad planes to be hit.
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

    /**
    Check if the ray hits the bounding box.

    # Examples

    ```
    use raytracing::core::{AxisAlignedBoundingBox, Interval, Ray, Vector3};

    let ray = Ray::new(Vector3::zero(), Vector3::one());
    let bbox = AxisAlignedBoundingBox::from_bounding_points(&Vector3::new(2., 2., 2.), &Vector3::new(3., 3., 3.));

    # assert!(!bbox.hit(&ray, &Interval::new(0., 1.999)));
    # assert!(bbox.hit(&ray, &Interval::new(0., 2.)));
    # assert!(!bbox.hit(&ray, &Interval::new(0., f32::NEG_INFINITY)));
    # assert!(bbox.hit(&ray, &Interval::new(0., f32::INFINITY)));
    ```
    */
    pub fn hit(&self, ray: &Ray, ray_interval: &Interval) -> bool {
        for i in 0..3 {
            // Hit each axis of the bounding box.
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
