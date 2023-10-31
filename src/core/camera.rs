use image::{Rgb, RgbImage};
use indicatif::ProgressBar;
use rand::Rng;
use std::fs;

use super::{Color3, HitRecord, Hittable, Interval, Point3, Ray, Vector3};
use crate::utils::{degree_to_radian, linear_to_gramma};

#[derive(Default)]
pub struct Camera {
    pub up: Vector3,       // Camera-relative "up" direction
    pub look_from: Point3, // Point camera is looking from
    pub look_at: Point3,   // Point camera is looking at
    u: Vector3,            // Camera frame basis vectors
    v: Vector3,            // Camera frame basis vectors
    w: Vector3,            // Camera frame basis vectors
    center: Point3,        // Camera center

    pub width: u32,        // Rendered image width in pixel count
    pub aspect_ratio: f32, // Ratio of image width over height
    pub vertical_fov: f32, // Vertical view angle (field of view)
    height: u32,           // Rendered image height

    pub defocus_angle: f32,      // Variation angle of rays through each pixel
    pub focus_dist: f32,         // Distance from camera lookfrom point to plane of perfect focus
    pub defocus_disk_u: Vector3, // Defocus disk horizontal radius
    pub defocus_disk_v: Vector3, // Defocus disk vertical radius

    pixel_delta_u: Vector3, // Offset to pixel to the right
    pixel_delta_v: Vector3, // Offset to pixel below
    pixel_origin: Point3,   // Location of pixel 0, 0

    pub samples_per_pixel: u8, // Count of random samples for each pixel
    pub max_ray_depth: u8,     // Maximum number of ray bounces into scene

    rng: rand::rngs::ThreadRng,
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            up: Vector3::new(0., 1., 0.),
            look_from: Point3::new(0., 0., -1.),
            look_at: Point3::zero(),
            width: 100,
            aspect_ratio: 1.,
            vertical_fov: 90.,
            defocus_angle: 0.,
            focus_dist: 10.,
            samples_per_pixel: 10,
            max_ray_depth: 10,
            rng: rand::thread_rng(),
            ..Default::default()
        }
    }

    fn initialize(&mut self) {
        // Calculate the u,v,w unit basis vectors for the camera coordinate frame.
        self.w = (self.look_from - &self.look_at).normolize();
        self.u = self.up.cross(&self.w).normolize();
        self.v = self.w.cross(&self.u);
        self.center = self.look_from;

        // Calculate the camera defocus disk basis vectors.
        let defocus_radius = self.focus_dist * f32::tan(degree_to_radian(self.defocus_angle / 2.));
        self.defocus_disk_u = self.u * defocus_radius;
        self.defocus_disk_v = self.v * defocus_radius;

        self.height = ((self.width as f32) / self.aspect_ratio) as u32;
        if self.height < 1 {
            self.height = 1;
        }
        // Determine viewport dimensions.
        let vertical_theta = degree_to_radian(self.vertical_fov);
        let viewport_height = 2. * f32::tan(vertical_theta / 2.) * self.focus_dist;
        let viewport_width = viewport_height * ((self.width as f32) / (self.height as f32));
        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = self.u * viewport_width; // Vector across viewport horizontal edge
        let viewport_v = -self.v * viewport_height; // Vector down viewport vertical edge

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        self.pixel_delta_u = viewport_u / (self.width as f32);
        self.pixel_delta_v = viewport_v / (self.height as f32);
        // Calculate the location of the upper left pixel.
        let viewport_top_left =
            self.center - &(self.w * self.focus_dist) - &(viewport_u / 2.) - &(viewport_v / 2.);
        self.pixel_origin = viewport_top_left + &((self.pixel_delta_u + &self.pixel_delta_v) * 0.5);
    }

    fn pixel_sample_square(&mut self) -> Vector3 {
        // Returns a random point in the square surrounding a pixel at the origin.
        let px = -0.5 + self.rng.gen_range(0.0..1.0);
        let py = -0.5 + self.rng.gen_range(0.0..1.0);
        return (self.pixel_delta_u * px) + &(self.pixel_delta_v * py);
    }

    fn defocus_disk_sample(&self) -> Vector3 {
        // Returns a random point in the camera defocus disk.
        let point = Vector3::random_in_unit_disk();
        return self.center + &(self.defocus_disk_u * point[0]) + &(self.defocus_disk_v * point[1]);
    }

    fn get_ray(&mut self, x: u32, y: u32) -> Ray {
        // Get a randomly-sampled camera ray for the pixel at location i,j, originating from
        // the camera defocus disk.
        let pixel_center = self.pixel_origin
            + &(self.pixel_delta_u * (x as f32))
            + &(self.pixel_delta_v * (y as f32));
        let pixel_sample = pixel_center + &self.pixel_sample_square();
        let ray_origin = if self.defocus_angle <= 0. {
            self.center
        } else {
            self.defocus_disk_sample()
        };
        let ray_direction = pixel_sample - &ray_origin;

        Ray::new(ray_origin, ray_direction)
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

    fn write_color(&self, x: u32, y: u32, buffer: &mut RgbImage, pixel_color: &Color3) {
        let intensity = Interval::new(0.000, 0.999);
        // Write the translated [0,255] value of each color component.
        let r = (256. * intensity.clamp(pixel_color.x)) as u8;
        let g = (256. * intensity.clamp(pixel_color.y)) as u8;
        let b = (256. * intensity.clamp(pixel_color.z)) as u8;

        buffer.put_pixel(x, y, Rgb([r, g, b]));
    }

    pub fn render(&mut self, world: &dyn Hittable, save_path: String) -> std::io::Result<()> {
        self.initialize();

        let path = std::path::Path::new(&save_path);
        let prefix = path.parent().unwrap();
        fs::create_dir_all(prefix)?;

        let bar = ProgressBar::new(u64::from(self.height));
        let mut buffer = RgbImage::new(self.width.into(), self.height.into());
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

                self.write_color(x, y, &mut buffer, &color);
            }
            bar.inc(1);
        }

        buffer.save(path).unwrap();
        bar.finish();
        Ok(())
    }
}
