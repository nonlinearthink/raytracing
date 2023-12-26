use raytracing::core::{
    CameraBuilder, Color3, EmissiveMaterial, HittableList, LambertianMaterial, NoiseTexture,
    Point3, Quad, SolidColorTexture, Sphere, Vector3,
};
use std::rc::Rc;

fn main() {
    let mut world = HittableList::new();

    // Textures
    let noise_texture = Rc::new(NoiseTexture::new_with_marble_effect(4.));

    // Materials
    let emissive_material = Rc::new(EmissiveMaterial::new(Rc::new(SolidColorTexture::new(
        4., 4., 4.,
    ))));

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
    world.add(Rc::new(Sphere::new(
        Point3::new(0., 7., 0.),
        2.,
        emissive_material.clone(),
    )));
    world.add(Rc::new(Quad::new(
        Point3::new(3., 1., -2.),
        Vector3::new(2., 0., 0.),
        Vector3::new(0., 2., 0.),
        emissive_material.clone(),
    )));

    // Camera
    let mut camera = CameraBuilder::default()
        .position(Point3::new(26., 3., 6.))
        .target(Point3::new(0., 2., 0.))
        .width(400)
        .aspect(16. / 9.)
        .fov(20.)
        .background(Color3::zero())
        .samples_per_pixel(100)
        .max_ray_depth(10)
        .build()
        .unwrap();
    camera
        .render(&world, "out/lights-demo.ppm".to_owned())
        .err();
}
