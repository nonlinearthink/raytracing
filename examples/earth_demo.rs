use std::rc::Rc;

use tiny_raytracer::core::{
    CameraBuilder, Color3, HittableList, ImageTexture, LambertianMaterial, Point3, Sphere,
};

fn main() {
    let mut world = HittableList::new();

    // Textures
    let earth_texture = Rc::new(ImageTexture::new("assets/earthmap.jpg".to_owned()).unwrap());

    // Materials
    let earth_surface = Rc::new(LambertianMaterial::new(earth_texture));

    // Primitives
    let globe = Rc::new(Sphere::new(Point3::new(0., 0., 0.), 2., earth_surface));
    world.add(globe);

    // Camera
    let mut camera = CameraBuilder::default()
        .position(Point3::new(0., 0., 12.))
        .target(Point3::new(0., 0., 0.))
        .width(400)
        .aspect(16. / 9.)
        .fov(20.)
        .background(Color3::new(0.7, 0.8, 1.))
        .samples_per_pixel(30)
        .max_ray_depth(10)
        .build()
        .unwrap();
    camera.render(&world, "out/earth-demo.ppm".to_owned()).err();
}
