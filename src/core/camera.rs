use rand::Rng;
use std::{fs, fs::File, io::Write};

use super::{Color3, HitRecord, Hittable, Interval, Point3, Ray, Vector3};
use crate::utils::linear_to_gramma;

#[derive(Default)]
pub struct Camera {
    pub width: u16,
    pub aspect_ratio: f32,
    height: u16,
    pub samples_per_pixel: u8,
    pub max_ray_depth: u8,
    rng: rand::rngs::ThreadRng,
    center: Point3,
    pixel_delta_u: Vector3,
    pixel_delta_v: Vector3,
    pixel_origin: Point3,
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            width: 100,
            aspect_ratio: 1.,
            samples_per_pixel: 10,
            max_ray_depth: 10,
            rng: rand::thread_rng(),
            ..Default::default()
        }
    }

    fn initialize(&mut self) {
        self.height = (f32::from(self.width) / self.aspect_ratio) as u16;
        // Calculate the image height, and ensure that it's at least 1.
        if self.height < 1 {
            self.height = 1;
        }

        // Camera
        let focal_length = 1.;
        let viewport_height = 2.;
        let viewport_width = viewport_height * (f32::from(self.width) / f32::from(self.height));
        self.center = Point3::zero();

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = Vector3::new(viewport_width, 0., 0.);
        let viewport_v = Vector3::new(0., -viewport_height, 0.);

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        self.pixel_delta_u = viewport_u / f32::from(self.width);
        self.pixel_delta_v = viewport_v / f32::from(self.height);

        // Calculate the location of the upper left pixel.
        let viewport_top_left = self.center
            - &Vector3::new(0., 0., focal_length)
            - &(viewport_u / 2.)
            - &(viewport_v / 2.);
        self.pixel_origin = viewport_top_left + &((self.pixel_delta_u + &self.pixel_delta_v) * 0.5);
    }

    fn pixel_sample_square(&mut self) -> Vector3 {
        // Returns a random point in the square surrounding a pixel at the origin.
        let px = -0.5 + self.rng.gen_range(0.0..1.0);
        let py = -0.5 + self.rng.gen_range(0.0..1.0);
        return (self.pixel_delta_u * px) + &(self.pixel_delta_v * py);
    }

    fn get_ray(&mut self, x: u16, y: u16) -> Ray {
        // Get a randomly sampled camera ray for the pixel at location x,y.
        let pixel_center = self.pixel_origin
            + &(self.pixel_delta_u * f32::from(x))
            + &(self.pixel_delta_v * f32::from(y));
        let pixel_sample = pixel_center + &self.pixel_sample_square();
        let ray_direction = pixel_sample - &self.center;

        Ray::new(self.center, ray_direction)
    }

    fn ray_color(&mut self, ray: &Ray, world: &dyn Hittable, ray_depth: u8) -> Color3 {
        // If we've exceeded the ray bounce limit, no more light is gathered.
        if ray_depth <= 0 {
            return Color3::zero();
        }

        // Meshes
        let mut record: HitRecord = HitRecord::new();
        // Fixing shadow acne by setting the nearest surface to 0.001
        if world.hit(ray, &Interval::new(0.001, f32::INFINITY), &mut record) {
            let material = record
                .material
                .as_ref()
                .expect("material should not be None if hitted.");

            let mut ray_scattered = Ray::new(Point3::zero(), Vector3::zero());
            let mut attenuation = Color3::zero();
            if material.scatter(ray, &record, &mut attenuation, &mut ray_scattered) {
                return attenuation * &self.ray_color(&ray_scattered, world, ray_depth - 1);
            }
            return Color3::zero();
        }

        // Sky
        let unit_direction: Vector3 = ray.direction.normolize();
        let a = 0.5 * (unit_direction.y + 1.);
        Color3::new(1.0, 1.0, 1.0) * (1. - a) + &(Color3::new(0.5, 0.7, 1.0) * a)
    }

    fn write_color(&self, file: &mut File, pixel_color: &Color3) {
        let intensity = Interval::new(0.000, 0.999);
        // Write the translated [0,255] value of each color component.
        let r = (256. * intensity.clamp(pixel_color.x)) as u8;
        let g = (256. * intensity.clamp(pixel_color.y)) as u8;
        let b = (256. * intensity.clamp(pixel_color.z)) as u8;

        write!(file, "{} {} {}\n", r, g, b).unwrap();
    }

    pub fn render(&mut self, world: &dyn Hittable) -> std::io::Result<()> {
        self.initialize();

        let path = std::path::Path::new("out/scene.ppm");
        let prefix = path.parent().unwrap();
        fs::create_dir_all(prefix)?;

        let mut ppm = File::create(path)?;
        write!(ppm, "P3\n{} {}\n255\n", self.width, self.height)?;
        for y in 0..self.height {
            for x in 0..self.width {
                let mut color = Color3::zero();
                for _ in 0..self.samples_per_pixel {
                    let ray = self.get_ray(x, y);
                    color += &self.ray_color(&ray, world, self.max_ray_depth);
                }

                let scale = 1. / f32::from(self.samples_per_pixel);
                color.x *= scale;
                color.y *= scale;
                color.z *= scale;

                // Apply the linear to gamma transform.
                color.x = linear_to_gramma(color.x);
                color.y = linear_to_gramma(color.y);
                color.z = linear_to_gramma(color.z);

                self.write_color(&mut ppm, &color);
            }
        }

        Ok(())
    }
}
