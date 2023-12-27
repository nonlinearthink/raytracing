use super::{
    Color3, HitRecord, Hittable, HittablePDF, Interval, Point3, ProbabilityDensityFunction, Ray,
    Vector3,
};
use crate::utils::{deg_to_rad, linear_to_gramma, PPMImage};
use derive_builder::Builder;
use indicatif::ProgressBar;
use rand::Rng;
use std::{
    ops::{Add, Div, Mul, Neg, Sub},
    rc::Rc,
    time,
};

#[derive(Default, Builder)]
pub struct Camera {
    /// Point camera is looking from
    #[builder(default = "Point3::new(0., 0., -1.)")]
    pub position: Point3,

    /// Point camera is looking at
    #[builder(default = "Point3::zero()")]
    pub target: Point3,

    /// Camera-relative "up" direction
    #[builder(default = "Vector3::up()")]
    pub up: Vector3,

    /// Camera frame basis vectors
    #[builder(setter(skip))]
    u: Vector3,

    /// Camera frame basis vectors
    #[builder(setter(skip))]
    v: Vector3,

    /// Camera frame basis vectors
    #[builder(setter(skip))]
    w: Vector3,

    /// Variation angle of rays through each pixel
    #[builder(default = "0.")]
    pub defocus_angle: f32,

    /// Distance from camera lookfrom point to plane of perfect focus
    #[builder(default = "10.")]
    pub focus_dist: f32,

    /// Defocus disk horizontal radius
    #[builder(setter(skip))]
    defocus_disk_u: Vector3,

    /// Defocus disk vertical radius
    #[builder(setter(skip))]
    defocus_disk_v: Vector3,

    /// Rendered image width in pixel count
    #[builder(default = "400")]
    pub width: u32,

    /// Rendered image height
    #[builder(setter(skip))]
    height: u32,

    /// Ratio of image width over height
    #[builder(default = "1.")]
    pub aspect: f32,

    /// Vertical view angle (field of view)
    #[builder(default = "20.")]
    pub fov: f32,

    /// Offset to pixel to the right
    #[builder(setter(skip))]
    pixel_delta_u: Vector3,

    /// Offset to pixel below
    #[builder(setter(skip))]
    pixel_delta_v: Vector3,

    // Location of pixel 0, 0
    #[builder(setter(skip))]
    pixel_origin: Point3,

    /// Count of random samples for each pixel
    #[builder(default = "20")]
    pub samples_per_pixel: u32,

    /// Sqrt of self.samples_per_pixel
    #[builder(setter(skip))]
    sqrt_spp: u32,

    /// Square of self.sqrt_spp
    #[builder(setter(skip))]
    square_sqrt_spp: f32,

    /// Reciprocal of self.sqrt_spp
    #[builder(setter(skip))]
    reciprocal_sqrt_spp: f32,

    /// Maximum number of ray bounces into scene
    #[builder(default = "10")]
    pub max_ray_depth: u8,

    /// Scene background color
    #[builder(default = "Color3::one()")]
    pub background: Color3,

    /// Rand generator
    #[builder(setter(skip))]
    rng: rand::rngs::ThreadRng,
}

impl Camera {
    /// Create a default camera
    pub fn new() -> Self {
        Self {
            position: Point3::new(0., 0., -1.),
            target: Point3::zero(),
            up: Vector3::up(),
            defocus_angle: 0.,
            focus_dist: 10.,
            width: 400,
            aspect: 1.,
            fov: 20.,
            samples_per_pixel: 20,
            max_ray_depth: 10,
            background: Color3::new(0.7, 0.8, 1.),
            rng: rand::thread_rng(),
            ..Default::default()
        }
    }

    fn initialize(&mut self) {
        // Calculate the u,v,w unit basis vectors for the camera coordinate frame.
        self.w = (&self.position - &self.target).normolize();
        self.u = self.up.cross(&self.w).normolize();
        self.v = self.w.cross(&self.u);

        // Calculate the camera defocus disk basis vectors.
        let defocus_radius = self.focus_dist * f32::tan(deg_to_rad(self.defocus_angle / 2.));
        self.defocus_disk_u = &self.u * defocus_radius;
        self.defocus_disk_v = &self.v * defocus_radius;

        self.height = ((self.width as f32) / self.aspect) as u32;
        if self.height < 1 {
            self.height = 1;
        }
        // Determine viewport dimensions.
        let vertical_theta = deg_to_rad(self.fov);
        let viewport_height = 2. * f32::tan(vertical_theta / 2.) * self.focus_dist;
        let viewport_width = viewport_height * ((self.width as f32) / (self.height as f32));
        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = &self.u * viewport_width; // Vector across viewport horizontal edge
        let viewport_v = &self.v.neg() * viewport_height; // Vector down viewport vertical edge

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        self.pixel_delta_u = &viewport_u / (self.width as f32);
        self.pixel_delta_v = &viewport_v / (self.height as f32);
        // Calculate the location of the upper left pixel.
        let viewport_top_left = &self
            .position
            .sub(&self.w.mul(self.focus_dist))
            .sub(&(&viewport_u / 2.))
            .sub(&(&viewport_v / 2.));
        self.pixel_origin =
            viewport_top_left.add(&self.pixel_delta_u.add(&self.pixel_delta_v).mul(0.5));

        self.sqrt_spp = f32::sqrt(self.samples_per_pixel as f32) as u32;
        self.square_sqrt_spp = (self.sqrt_spp * self.sqrt_spp) as f32;
        self.reciprocal_sqrt_spp = 1. / self.sqrt_spp as f32;
    }

    fn pixel_sample_square(&mut self, sub_x: u32, sub_y: u32) -> Vector3 {
        // Returns a random point in the square surrounding a pixel at the origin.
        let px = -0.5 + (sub_x as f32 + self.rng.gen::<f32>()) * self.reciprocal_sqrt_spp;
        let py = -0.5 + (sub_y as f32 + self.rng.gen::<f32>()) * self.reciprocal_sqrt_spp;
        return &self.pixel_delta_u.mul(px) + &self.pixel_delta_v.mul(py);
    }

    fn defocus_disk_sample(&self) -> Vector3 {
        // Returns a random point in the camera defocus disk.
        let point = Vector3::random_in_unit_disk();
        return self
            .position
            .add(&self.defocus_disk_u.mul(point[0]))
            .add(&self.defocus_disk_v.mul(point[1]));
    }

    fn get_ray(&mut self, x: u32, y: u32, sub_x: u32, sub_y: u32) -> Ray {
        // Get a randomly-sampled camera ray for the pixel at location i,j, originating from
        // the camera defocus disk.
        let pixel_center = self
            .pixel_origin
            .add(&self.pixel_delta_u.mul(x as f32))
            .add(&self.pixel_delta_v.mul(y as f32));
        let pixel_sample = &pixel_center + &self.pixel_sample_square(sub_x, sub_y);
        let ray_origin = if self.defocus_angle <= 0. {
            self.position
        } else {
            self.defocus_disk_sample()
        };
        let ray_direction = &pixel_sample - &ray_origin;
        let ray_time = self.rng.gen::<f32>();

        Ray::new_with_time(ray_origin, ray_direction, ray_time)
    }

    fn ray_color(
        &mut self,
        ray: &Ray,
        world: Rc<dyn Hittable>,
        lights: Option<Rc<dyn Hittable>>,
        ray_depth: u8,
    ) -> Color3 {
        // If we've exceeded the ray bounce limit, no more light is gathered.
        if ray_depth <= 0 {
            return Color3::zero();
        }

        let mut record = HitRecord::new();
        // Fixing shadow acne by setting the nearest surface to 0.001
        if !world.hit(ray, &Interval::new(0.001, f32::INFINITY), &mut record) {
            return self.background;
        }

        let HitRecord {
            uv: Some(uv),
            point: Some(point),
            material: Some(ref material),
            ..
        } = record
        else {
            return Color3::zero();
        };

        let emission_color = material.emitted(ray, &record, &uv, &point);

        let mut ray_scattered = Ray::new(Point3::zero(), Vector3::zero());
        let mut attenuation = Color3::zero();
        let mut pdf = 0.;
        if !material.scatter(ray, &record, &mut attenuation, &mut ray_scattered, &mut pdf) {
            return emission_color;
        }

        if let Some(ref lights_some) = lights {
            let light_pdf = HittablePDF::new(lights_some.clone(), point);
            ray_scattered = Ray::new_with_time(point, light_pdf.generate(), ray.time);
            pdf = light_pdf.value(&ray_scattered.direction);
        }

        let scattering_pdf = material.scattering_pdf(ray, &record, &mut ray_scattered);

        let sample_color = self.ray_color(&ray_scattered, world, lights, ray_depth - 1);
        let scatter_color = attenuation.mul(scattering_pdf).mul(&sample_color).div(pdf);

        &emission_color + &scatter_color
    }

    pub fn render(
        &mut self,
        world: Rc<dyn Hittable>,
        lights: Option<Rc<dyn Hittable>>,
        save_path: String,
    ) -> std::io::Result<()> {
        self.initialize();

        let render_timer = time::Instant::now();
        let render_progress_bar = ProgressBar::new(u64::from(self.height));
        println!("Rendering:");

        let mut image = PPMImage::new(self.width.into(), self.height.into());
        for y in 0..self.height {
            for x in 0..self.width {
                let mut color = Color3::zero();
                for sub_y in 0..self.sqrt_spp {
                    for sub_x in 0..self.sqrt_spp {
                        let ray = self.get_ray(x, y, sub_x, sub_y);
                        color += &self.ray_color(
                            &ray,
                            world.clone(),
                            lights.clone(),
                            self.max_ray_depth,
                        );
                    }
                }

                // Replace NaN components with zero.
                if color.x != color.x {
                    color.x = 0.;
                }
                if color.y != color.y {
                    color.y = 0.;
                }
                if color.z != color.z {
                    color.z = 0.;
                }

                // Divide the color by the number of samples and gamma-correct for gamma=2.0.
                let scale = 1. / self.square_sqrt_spp;
                color.x = linear_to_gramma(color.x * scale);
                color.y = linear_to_gramma(color.y * scale);
                color.z = linear_to_gramma(color.z * scale);

                // Write the translated [0,255] value of each color component.
                let intensity = Interval::new(0.000, 0.999);
                let r = (256. * intensity.clamp(color.x)) as u8;
                let g = (256. * intensity.clamp(color.y)) as u8;
                let b = (256. * intensity.clamp(color.z)) as u8;

                image.write_color(r, g, b).unwrap();
            }
            render_progress_bar.inc(1);
        }

        image.save(save_path.clone()).unwrap();

        render_progress_bar.finish();
        let render_cost = render_timer.elapsed();
        println!("Render Cost: {:?}", render_cost);

        println!("{} generated.", save_path);

        Ok(())
    }
}
