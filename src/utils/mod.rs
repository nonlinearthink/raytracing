mod ppm;

pub use ppm::*;

/**
Convert degree to radian.

# Examples

```
use raytracing::utils::deg_to_rad;

let radian = deg_to_rad(45.);
# assert_eq!(radian, 0.7853981633974483);
```
*/
#[inline]
pub fn deg_to_rad(degree: f32) -> f32 {
    degree * std::f32::consts::PI / 180.0
}

/**
Convert degree to radian.

# Examples

```
use raytracing::utils::rad_to_deg;

let degree = rad_to_deg(0.7853981633974483);
# assert_eq!(degree, 45.);
```
*/
#[inline]
pub fn rad_to_deg(radian: f32) -> f32 {
    radian * 180. / std::f32::consts::PI
}

/**
Convert linear component to gramma component.

# Examples

```
use raytracing::utils::linear_to_gramma;

let gramma_component = linear_to_gramma(10.);
# assert_eq!(gramma_component, 3.1622776601683795);
```
*/
#[inline]
pub fn linear_to_gramma(linear_component: f32) -> f32 {
    f32::sqrt(linear_component)
}

/**
Convert gramma component to linear component.

# Examples

```
use raytracing::utils::gramma_to_linear;

let linear_component = gramma_to_linear(3.1622776601683795);
# assert_eq!(linear_component, 10.);
```
*/
#[inline]
pub fn gramma_to_linear(gramma_component: f32) -> f32 {
    gramma_component.powi(2)
}
