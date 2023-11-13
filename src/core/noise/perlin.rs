use crate::core::{Point3, Vector3};
use rand::Rng;

const PERLIN_LUT_LENGTH: usize = 256;

#[derive(Debug)]
pub struct Perlin {
    values: Vec<Vector3>,
    x_permutation: [usize; PERLIN_LUT_LENGTH],
    y_permutation: [usize; PERLIN_LUT_LENGTH],
    z_permutation: [usize; PERLIN_LUT_LENGTH],
}

impl Perlin {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let mut values: Vec<Vector3> = vec![];
        for _ in 0..PERLIN_LUT_LENGTH {
            values.push(Vector3::random(-1., 1., &mut rng).normolize());
        }

        Self {
            values,
            x_permutation: Self::generate_permutation(),
            y_permutation: Self::generate_permutation(),
            z_permutation: Self::generate_permutation(),
        }
    }

    pub fn noise(&self, point: &Point3) -> f32 {
        let u = point.x - f32::floor(point.x);
        let v = point.y - f32::floor(point.y);
        let w = point.z - f32::floor(point.z);

        let i = f32::floor(point.x) as i32;
        let j = f32::floor(point.y) as i32;
        let k = f32::floor(point.z) as i32;

        let mut regular_grid = [[[Vector3::zero(); 2]; 2]; 2];
        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    let x_perm = self.x_permutation[((i + di as i32) & 255) as usize];
                    let y_perm = self.y_permutation[((j + dj as i32) & 255) as usize];
                    let z_perm = self.z_permutation[((k + dk as i32) & 255) as usize];
                    let final_perm = x_perm ^ y_perm ^ z_perm;

                    regular_grid[di][dj][dk] = self.values[final_perm];
                }
            }
        }

        Perlin::trilinear_interpolation(&regular_grid, u, v, w)
    }

    fn do_permute(permutation: &mut [usize; PERLIN_LUT_LENGTH]) {
        let mut rng = rand::thread_rng();
        for i in (1..=permutation.len() - 1).rev() {
            let j = rng.gen_range(0..i);
            permutation.swap(i, j);
        }
    }

    fn generate_permutation() -> [usize; PERLIN_LUT_LENGTH] {
        let mut permutation = [0; PERLIN_LUT_LENGTH];
        for i in 0..PERLIN_LUT_LENGTH {
            permutation[i] = i;
        }

        Perlin::do_permute(&mut permutation);

        permutation
    }

    /// see: https://en.wikipedia.org/wiki/Trilinear_interpolation
    fn trilinear_interpolation(
        regular_grid: &[[[Vector3; 2]; 2]; 2],
        u: f32,
        v: f32,
        w: f32,
    ) -> f32 {
        // use a hermite cubic to round off the interpolation
        let roundu = u * u * (3. - 2. * u);
        let roundv = v * v * (3. - 2. * v);
        let roundw = w * w * (3. - 2. * w);

        let mut accumulate = 0.;
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let fi = i as f32;
                    let fj = j as f32;
                    let fk = k as f32;

                    let interp_u = fi * roundu + (1. - fi) * (1. - roundu);
                    let interp_v = fj * roundv + (1. - fj) * (1. - roundv);
                    let interp_w = fk * roundw + (1. - fk) * (1. - roundw);

                    let weight = Vector3::new(u - i as f32, v - j as f32, w - k as f32);

                    accumulate +=
                        interp_u * interp_v * interp_w * regular_grid[i][j][k].dot(&weight);
                }
            }
        }
        accumulate
    }

    pub fn turbulence(&self, point: &Point3, depth: usize) -> f32 {
        let mut accumulate = 0.;
        let mut weight = 1.;

        let mut temp_point = point.clone();
        for _ in 0..depth {
            accumulate += weight * self.noise(&temp_point);
            weight *= 0.5;
            temp_point *= 2.;
        }

        f32::abs(accumulate)
    }
}
