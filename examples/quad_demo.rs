use std::rc::Rc;

use tiny_raytracer::core::{
    Camera, HittableList, LambertianMaterial, Point3, Quadrilateral, SolidColorTexture, Vector3,
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

    // Quads
    world.add(Rc::new(Quadrilateral::new(
        Point3::new(-3., -2., 5.),
        Vector3::new(0., 0., -4.),
        Vector3::new(0., 4., 0.),
        left_red,
    )));
    world.add(Rc::new(Quadrilateral::new(
        Point3::new(-2., -2., 0.),
        Vector3::new(4., 0., 0.),
        Vector3::new(0., 4., 0.),
        back_green,
    )));
    world.add(Rc::new(Quadrilateral::new(
        Point3::new(3., -2., 1.),
        Vector3::new(0., 0., 4.),
        Vector3::new(0., 4., 0.),
        right_blue,
    )));
    world.add(Rc::new(Quadrilateral::new(
        Point3::new(-2., 3., 1.),
        Vector3::new(4., 0., 0.),
        Vector3::new(0., 0., 4.),
        upper_orange,
    )));
    world.add(Rc::new(Quadrilateral::new(
        Point3::new(-2., -3., 5.),
        Vector3::new(4., 0., 0.),
        Vector3::new(0., 0., -4.),
        lower_teal,
    )));

    let mut camera = Camera::new();

    camera.look_from = Point3::new(0., 0., 9.);
    camera.look_at = Point3::new(0., 0., 0.);

    camera.width = 400;
    camera.aspect_ratio = 1.0;
    camera.vertical_fov = 80.;

    camera.samples_per_pixel = 100;
    camera.max_ray_depth = 50;

    camera.render(&world, "out/quad-demo.ppm".to_owned()).err();
}
