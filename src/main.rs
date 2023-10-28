use std::time;

use rst_raytrace::core::{Camera, HittableList, Point3, Sphere};

fn main() {
    // World
    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Point3::new(0., 0., -1.), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0., -100.5, -1.), 100.)));

    let mut camera = Camera::new();

    camera.width = 400;
    camera.aspect_ratio = 16. / 9.;

    let render_timer = time::Instant::now();
    camera.render(&world).err();
    let render_cost = render_timer.elapsed();
    println!("Render Cost: {:?}", render_cost);
}
