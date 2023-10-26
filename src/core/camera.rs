use std::{fs, fs::File, io::Write};

use super::{Color3, HitRecord, Hittable, Interval, Point3, Ray, Vector3};

#[derive(Default)]
pub struct Camera {
    pub width: u16,
    pub aspect_ratio: f32,
    height: u16,
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

    fn ray_color(&self, ray: &Ray, world: &dyn Hittable) -> Color3 {
        // Meshes
        let mut record: HitRecord = HitRecord::new();
        if world.hit(ray, &Interval::new(0., f32::INFINITY), &mut record) {
            let normal = record.normal.expect("normal should not be None if hitted.");
            return (normal + &Color3::one()) * 0.5;
        }

        // Sky
        let unit_direction: Vector3 = ray.direction.normolize();
        let a = 0.5 * (unit_direction.y + 1.);
        Color3::new(1.0, 1.0, 1.0) * (1. - a) + &(Color3::new(0.5, 0.7, 1.0) * a)
    }

    fn write_color(&self, file: &mut File, pixel_color: &Color3) {
        // Write the translated [0,255] value of each color component.
        let ir = (255.999 * pixel_color.x) as u8;
        let ig = (255.999 * pixel_color.y) as u8;
        let ib = (255.999 * pixel_color.z) as u8;

        write!(file, "{} {} {}\n", ir, ig, ib).unwrap();
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
                let pixel_center = self.pixel_origin
                    + &(self.pixel_delta_u * f32::from(x))
                    + &(self.pixel_delta_v * f32::from(y));
                let ray_direction = pixel_center - &self.center;
                let ray = Ray::new(self.center, ray_direction);

                let color = self.ray_color(&ray, world);

                self.write_color(&mut ppm, &color);
            }
        }

        println!("Render Finished.");
        Ok(())
    }
}
