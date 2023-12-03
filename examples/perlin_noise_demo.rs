use std::rc::Rc;

use raytracing::core::{
    CameraBuilder, Color3, HittableList, LambertianMaterial, NoiseTexture, Point3, Sphere,
};

struct SceneOptions {
    marble_effect: bool,
}

fn main() {
    let options = SceneOptions {
        marble_effect: false,
    };

    let mut world = HittableList::new();

    // Textures
    let noise_texture = if options.marble_effect {
        Rc::new(NoiseTexture::new_with_marble_effect(4.))
    } else {
        Rc::new(NoiseTexture::new(4.))
    };

    // Primitives
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

    // Camera
    let mut camera = CameraBuilder::default()
        .position(Point3::new(13., 2., 3.))
        .target(Point3::zero())
        .width(400)
        .aspect(16. / 9.)
        .fov(20.)
        .background(Color3::new(0.7, 0.8, 1.))
        .samples_per_pixel(30)
        .max_ray_depth(10)
        .build()
        .unwrap();
    camera
        .render(&world, "out/perlin-noise-demo.ppm".to_owned())
        .err();
}
