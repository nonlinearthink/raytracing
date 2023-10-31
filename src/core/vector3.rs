use rand::Rng;
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug, Copy, Clone, Default)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vector3 {
        Vector3 { x, y, z }
    }

    pub fn zero() -> Vector3 {
        Vector3 {
            x: 0.,
            y: 0.,
            z: 0.,
        }
    }

    pub fn one() -> Vector3 {
        Vector3 {
            x: 1.,
            y: 1.,
            z: 1.,
        }
    }

    pub fn random(min: f32, max: f32, rng: &mut rand::rngs::ThreadRng) -> Vector3 {
        let x = rng.gen_range(min..=max);
        let y = rng.gen_range(min..=max);
        let z = rng.gen_range(min..=max);

        Vector3::new(x, y, z)
    }

    pub fn random_in_unit_sphere() -> Vector3 {
        let mut rng = rand::thread_rng();
        let theta = rng.gen_range(0.0..(std::f32::consts::PI * 2.));
        let phi = rng.gen_range(0.0..std::f32::consts::PI);
        let radius = rng.gen_range(0.0..1.0);
        let x = radius * theta.sin() * phi.cos();
        let y = radius * theta.sin() * phi.sin();
        let z = radius * theta.cos();
        Vector3::new(x, y, z)
    }

    pub fn random_in_unit_disk() -> Vector3 {
        let mut rng = rand::thread_rng();
        let theta = rng.gen_range(0.0..(std::f32::consts::PI * 2.));
        let radius = rng.gen_range(0.0..1.0);
        let x = radius * theta.cos();
        let y = radius * theta.sin();
        Vector3::new(x, y, 0.)
    }

    pub fn random_unit_vector() -> Vector3 {
        Vector3::random_in_unit_sphere().normolize()
    }

    pub fn random_on_hemisphere(normal: &Vector3) -> Vector3 {
        let vector_in_unit_sphere = Vector3::random_unit_vector();
        if vector_in_unit_sphere.dot(normal) > 0. {
            vector_in_unit_sphere
        } else {
            -vector_in_unit_sphere
        }
    }

    pub fn dot(&self, rhs: &Vector3) -> f32 {
        self[0] * rhs[0] + self[1] * rhs[1] + self[2] * rhs[2]
    }

    pub fn cross(&self, rhs: &Vector3) -> Vector3 {
        Vector3::new(
            self[1] * rhs[2] - self[2] * rhs[1],
            self[2] * rhs[0] - self[0] * rhs[2],
            self[0] * rhs[1] - self[1] * rhs[0],
        )
    }

    pub fn length_squared(&self) -> f32 {
        self.dot(self)
    }

    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn normolize(&self) -> Vector3 {
        self.div(self.length())
    }

    pub fn reflect(&self, normal: &Vector3) -> Vector3 {
        self.sub(&normal.mul(self.dot(normal)).mul(2.))
    }

    pub fn refract(&self, normal: &Vector3, refraction_ratio: f32) -> Vector3 {
        let cos_theta = f32::min(self.neg().dot(normal), 1.0);
        let ray_out_perpendicular = self.add(&normal.mul(cos_theta)) * refraction_ratio;
        let ray_out_parallel = normal.mul(-f32::sqrt(f32::abs(
            1.0 - ray_out_perpendicular.length_squared(),
        )));
        return ray_out_perpendicular + &ray_out_parallel;
    }

    pub fn equals(&self, rhs: &Self) -> bool {
        f32::abs(self.x - rhs.x) < f32::EPSILON
            && f32::abs(self.y - rhs.y) < f32::EPSILON
            && f32::abs(self.z - rhs.z) < f32::EPSILON
    }

    pub fn equals_zero(&self) -> bool {
        // Return true if the vector is close to zero in all dimensions.
        (self.x.abs() < f32::EPSILON)
            && (self.y.abs() < f32::EPSILON)
            && (self.z.abs() < f32::EPSILON)
    }
}

impl std::ops::Index<usize> for Vector3 {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Out of bounds access"),
        }
    }
}

impl std::ops::Neg for Vector3 {
    type Output = Vector3;

    fn neg(self) -> Self::Output {
        Vector3::new(-self.x, -self.y, -self.z)
    }
}

impl std::ops::Add<&Vector3> for Vector3 {
    type Output = Vector3;

    fn add(self, rhs: &Vector3) -> Self::Output {
        Vector3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl std::ops::Add<f32> for Vector3 {
    type Output = Vector3;

    fn add(self, rhs: f32) -> Self::Output {
        Vector3::new(self.x + rhs, self.y + rhs, self.z + rhs)
    }
}

impl std::ops::Sub<&Vector3> for Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: &Vector3) -> Self::Output {
        Vector3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl std::ops::Sub<f32> for Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: f32) -> Self::Output {
        Vector3::new(self.x - rhs, self.y - rhs, self.z - rhs)
    }
}

impl std::ops::Mul<&Vector3> for Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: &Vector3) -> Self::Output {
        Vector3::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}

impl std::ops::Mul<f32> for Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: f32) -> Self::Output {
        Vector3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl std::ops::Div<&Vector3> for Vector3 {
    type Output = Vector3;

    fn div(self, rhs: &Vector3) -> Self::Output {
        Vector3::new(self.x / rhs.x, self.y / rhs.y, self.z / rhs.z)
    }
}

impl std::ops::Div<f32> for Vector3 {
    type Output = Vector3;

    fn div(self, rhs: f32) -> Self::Output {
        Vector3::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl std::ops::AddAssign<&Vector3> for Vector3 {
    fn add_assign(&mut self, rhs: &Vector3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl std::ops::AddAssign<f32> for Vector3 {
    fn add_assign(&mut self, rhs: f32) {
        self.x += rhs;
        self.y += rhs;
        self.z += rhs;
    }
}

impl std::ops::SubAssign<&Vector3> for Vector3 {
    fn sub_assign(&mut self, rhs: &Vector3) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl std::ops::SubAssign<f32> for Vector3 {
    fn sub_assign(&mut self, rhs: f32) {
        self.x -= rhs;
        self.y -= rhs;
        self.z -= rhs;
    }
}

impl std::ops::MulAssign<&Vector3> for Vector3 {
    fn mul_assign(&mut self, rhs: &Vector3) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}

impl std::ops::MulAssign<f32> for Vector3 {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl std::ops::DivAssign<&Vector3> for Vector3 {
    fn div_assign(&mut self, rhs: &Vector3) {
        self.x /= rhs.x;
        self.y /= rhs.y;
        self.z /= rhs.z;
    }
}

impl std::ops::DivAssign<f32> for Vector3 {
    fn div_assign(&mut self, rhs: f32) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}
