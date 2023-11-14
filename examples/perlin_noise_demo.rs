use std::rc::Rc;

use tiny_raytracer::core::{
    Camera, HittableList, LambertianMaterial, NoiseTexture, Point3, Sphere,
};

struct SceneOptions {
    marble_effect: bool,
}

fn main() {
    let options = SceneOptions {
        marble_effect: false,
    };

    let mut world = HittableList::new();

    let noise_texture = if options.marble_effect {
        Rc::new(NoiseTexture::new_with_marble_effect(4.))
    } else {
        Rc::new(NoiseTexture::new(4.))
    };

    world.add(Rc::new(Sphere::new(
        Point3::new(0., -1000., 0.),
        1000.,
        Rc::new(LambertianMaterial::new(noise_texture.clone())),
    )));

    world.add(Rc::new(Sphere::new(
        Point3::new(0., 2., 0.),
        2.,
        Rc::new(LambertianMaterial::new(noise_texture.clone())),
    )));

    let mut camera = Camera::new();

    camera.look_from = Point3::new(13., 2., 3.);
    camera.look_at = Point3::new(0., 0., 0.);

    camera.width = 400;
    camera.aspect_ratio = 16. / 9.;
    camera.vertical_fov = 20.;

    camera.samples_per_pixel = 100;
    camera.max_ray_depth = 50;

    camera
        .render(&world, "out/perlin-noise-demo.ppm".to_owned())
        .err();
}
