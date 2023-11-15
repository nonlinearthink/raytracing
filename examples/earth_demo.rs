use std::rc::Rc;

use tiny_raytracer::core::{
    CameraBuilder, HittableList, ImageTexture, LambertianMaterial, Point3, Sphere,
};

fn main() {
    let mut world = HittableList::new();

    let earth_texture =
        Rc::new(ImageTexture::new("resource/images/earthmap.jpg".to_owned()).unwrap());
    let earth_surface = Rc::new(LambertianMaterial::new(earth_texture));
    let globe = Rc::new(Sphere::new(Point3::new(0., 0., 0.), 2., earth_surface));
    world.add(globe);

    let mut camera = CameraBuilder::default()
        .position(Point3::new(0., 0., 12.))
        .target(Point3::new(0., 0., 0.))
        .width(400)
        .aspect(16. / 9.)
        .fov(20.)
        .samples_per_pixel(100)
        .max_ray_depth(10)
        .build()
        .unwrap();

    camera.render(&world, "out/earth-demo.ppm".to_owned()).err();
}
