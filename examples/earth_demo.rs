use std::rc::Rc;

use tiny_raytracer::core::{
    Camera, HittableList, ImageTexture, LambertianMaterial, Point3, Sphere,
};

fn main() {
    let mut world = HittableList::new();

    let earth_texture =
        Rc::new(ImageTexture::new("resource/images/earthmap.jpg".to_owned()).unwrap());
    let earth_surface = Rc::new(LambertianMaterial::new(earth_texture));
    let globe = Rc::new(Sphere::new(Point3::new(0., 0., 0.), 2., earth_surface));
    world.add(globe);

    let mut camera = Camera::new();

    camera.look_from = Point3::new(0., 0., 12.);
    camera.look_at = Point3::new(0., 0., 0.);

    camera.width = 400;
    camera.aspect_ratio = 16. / 9.;
    camera.vertical_fov = 20.;

    camera.samples_per_pixel = 30;
    camera.max_ray_depth = 10;

    camera.render(&world, "out/earth-demo.ppm".to_owned()).err();
}
