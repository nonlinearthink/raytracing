use rand::{rngs::ThreadRng, Rng};

use crate::core::Point3;

const PERLIN_IMAGE_SIZE: usize = 256;

pub struct Perlin {
    rng: ThreadRng,
    data: [f32; PERLIN_IMAGE_SIZE],
    x_permute: [usize; PERLIN_IMAGE_SIZE],
    y_permute: [usize; PERLIN_IMAGE_SIZE],
    z_permute: [usize; PERLIN_IMAGE_SIZE],
}

impl Perlin {
    pub fn new() -> Perlin {
        let mut rng = rand::thread_rng();
        let mut data = [0.; PERLIN_IMAGE_SIZE];
        for i in 0..PERLIN_IMAGE_SIZE {
            data[i] = rng.gen::<f32>();
        }

        Perlin {
            rng,
            data,
            x_permute: Self::generate_permute(),
            y_permute: Self::generate_permute(),
            z_permute: Self::generate_permute(),
        }
    }

    pub fn noise(&self, point: Point3) {
        let i = ((4. * point.x) as usize) & 255;
        let j = ((4. * point.y) as usize) & 255;
        let k = ((4. * point.z) as usize) & 255;

        self.data[i ^ j ^ k];
    }

    fn permute(permute: &mut [usize; PERLIN_IMAGE_SIZE]) {
        let mut rng = rand::thread_rng();
        for i in (1..=permute.len() - 1).rev() {
            let j = rng.gen_range(0..i);
            permute.swap(i, j);
        }
    }

    fn generate_permute() -> [usize; PERLIN_IMAGE_SIZE] {
        let mut permute = [0; PERLIN_IMAGE_SIZE];
        for i in 0..PERLIN_IMAGE_SIZE {
            permute[i] = i;
        }

        Perlin::permute(&mut permute);

        permute
    }
}
