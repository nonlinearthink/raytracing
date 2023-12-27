use super::{Point3, Vector3};
use std::ops::{Add, Mul};

/**
Definition of ray

# Examples

```
use raytracing::core::{Point3, Ray, Vector3};

let origin = Point3::new(1., 2., 3.);
let direction = Vector3::new(4., 5., 6.);

let ray = Ray::new(origin, direction);
let target = ray.at(2.);

# assert_eq!(origin, Vector3::new(1., 2., 3.));
# assert_eq!(direction, Vector3::new(4., 5., 6.));
# assert_eq!(target, Vector3::new(9., 12., 15.));
```
*/
#[derive(Debug)]
pub struct Ray {
    /// Initial position of the ray emission.
    pub origin: Point3,
    /// Propagation direction of the ray emission.
    pub direction: Vector3,
    /// Time of the ray emission, relatived to the start of this frame, generally between 0 and 1.
    pub time: f32,
}

impl Ray {
    /// Create a new `Ray` with origin and direction.
    pub fn new(origin: Point3, direction: Vector3) -> Self {
        Self {
            origin,
            direction,
            time: 0.,
        }
    }

    /// Create a new `Ray` with origin, direction and time.
    pub fn new_with_time(origin: Point3, direction: Vector3, time: f32) -> Self {
        Self {
            origin,
            direction,
            time,
        }
    }

    /// Calculate the position of the ray after a certain time.
    pub fn at(&self, time: f32) -> Point3 {
        self.direction.mul(time).add(&self.origin)
    }
}
