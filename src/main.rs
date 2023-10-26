use rst_raytrace::core::{Color3, HitRecord, Hittable, HittableList, Point3, Ray, Sphere, Vector3};
use std::{
    fs::File,
    io::Write,
    ops::{Add, Div, Mul, Sub},
};

fn ray_color(ray: &Ray, world: &dyn Hittable) -> Color3 {
    let mut record: HitRecord = HitRecord::new();
    if world.hit(ray, 0., f32::INFINITY, &mut record) {
        let normal = record.normal.expect("normal should not be None if hitted.");
        return normal + &Color3::one() + 0.5;
    }

    let unit_direction: Vector3 = ray.direction.normolize();
    let a = 0.5 * (unit_direction.y + 1.);
    Color3::new(1.0, 1.0, 1.0)
        .mul(1. - a)
        .add(&Color3::new(0.5, 0.7, 1.0).mul(a))
}

fn write_color(file: &mut File, pixel_color: &Color3) {
    // Write the translated [0,255] value of each color component.
    let ir = (255.999 * pixel_color.x) as u8;
    let ig = (255.999 * pixel_color.y) as u8;
    let ib = (255.999 * pixel_color.z) as u8;

    write!(file, "{} {} {}\n", ir, ig, ib).unwrap();
}

fn render() -> std::io::Result<()> {
    let aspect_ratio: f32 = 16. / 9.;

    // Image
    let width: u16 = 400;
    let mut height: u16 = (f32::from(width) / aspect_ratio) as u16;
    // Calculate the image height, and ensure that it's at least 1.
    height = if height < 1 { 1 } else { height };

    // World
    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Point3::new(0., 0., -1.), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0., -100.5, -1.), 100.)));
    let world_box = Box::from(world);

    // Camera
    let focal_length = 1.;
    let viewport_height = 2.;
    let viewport_width = viewport_height * (f32::from(width) / f32::from(height));
    let camera_center = Point3::zero();

    // Calculate the vectors across the horizontal and down the vertical viewport edges.
    let viewport_u = Vector3::new(viewport_width, 0., 0.);
    let viewport_v = Vector3::new(0., -viewport_height, 0.);

    // Calculate the horizontal and vertical delta vectors from pixel to pixel.
    let pixel_delta_u = viewport_u.div(f32::from(width));
    let pixel_delta_v = viewport_v.div(f32::from(height));

    // Calculate the location of the upper left pixel.
    let viewport_top_left = camera_center
        .sub(&Vector3::new(0., 0., focal_length))
        .sub(&viewport_u.div(2.))
        .sub(&viewport_v.div(2.));
    let first_pixel_location = viewport_top_left.add(&pixel_delta_u.add(&pixel_delta_v).mul(0.5));

    // TODO: struct viewport

    let path = std::path::Path::new("out/scene.ppm");
    let prefix = path.parent().unwrap();
    std::fs::create_dir_all(prefix)?;

    let mut ppm = File::create(path)?;
    write!(ppm, "P3\n{} {}\n255\n", width, height)?;
    for y in 0..height {
        for x in 0..width {
            let pixel_center = first_pixel_location
                .add(&pixel_delta_u.mul(f32::from(x)))
                .add(&pixel_delta_v.mul(f32::from(y)));
            let ray_direction = pixel_center.sub(&camera_center);
            let ray = Ray::new(camera_center, ray_direction);

            let color = ray_color(&ray, &world);

            write_color(&mut ppm, &color);
        }
    }
    println!("PPM File Generated.");
    Ok(())
}

fn main() {
    render().err();
}
