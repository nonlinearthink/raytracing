use std::rc::Rc;

use raytracing::core::{
    CameraBuilder, Color3, HittableList, LambertianMaterial, Point3, Quad, SolidColorTexture,
    Vector3,
};

fn main() {
    let mut world = HittableList::new();

    // Materials
    let left_red = Rc::new(LambertianMaterial::new(Rc::new(
        SolidColorTexture::new_with_floats(1.0, 0.2, 0.2),
    )));
    let back_green = Rc::new(LambertianMaterial::new(Rc::new(
        SolidColorTexture::new_with_floats(0.2, 1.0, 0.2),
    )));
    let right_blue = Rc::new(LambertianMaterial::new(Rc::new(
        SolidColorTexture::new_with_floats(0.2, 0.2, 1.0),
    )));
    let upper_orange = Rc::new(LambertianMaterial::new(Rc::new(
        SolidColorTexture::new_with_floats(1.0, 0.5, 0.0),
    )));
    let lower_teal = Rc::new(LambertianMaterial::new(Rc::new(
        SolidColorTexture::new_with_floats(0.2, 0.8, 0.8),
    )));

    // Primitives
    world.add(Rc::new(Quad::new(
        Point3::new(-3., -2., 5.),
        Vector3::new(0., 0., -4.),
        Vector3::new(0., 4., 0.),
        left_red,
    )));
    world.add(Rc::new(Quad::new(
        Point3::new(-2., -2., 0.),
        Vector3::new(4., 0., 0.),
        Vector3::new(0., 4., 0.),
        back_green,
    )));
    world.add(Rc::new(Quad::new(
        Point3::new(3., -2., 1.),
        Vector3::new(0., 0., 4.),
        Vector3::new(0., 4., 0.),
        right_blue,
    )));
    world.add(Rc::new(Quad::new(
        Point3::new(-2., 3., 1.),
        Vector3::new(4., 0., 0.),
        Vector3::new(0., 0., 4.),
        upper_orange,
    )));
    world.add(Rc::new(Quad::new(
        Point3::new(-2., -3., 5.),
        Vector3::new(4., 0., 0.),
        Vector3::new(0., 0., -4.),
        lower_teal,
    )));

    // Camera
    let mut camera = CameraBuilder::default()
        .position(Point3::new(0., 0., 9.))
        .target(Point3::zero())
        .width(400)
        .aspect(1.0)
        .fov(80.)
        .background(Color3::new(0.7, 0.8, 1.))
        .samples_per_pixel(30)
        .max_ray_depth(10)
        .build()
        .unwrap();
    camera.render(&world, "out/quad-demo.ppm".to_owned()).err();
}
