use rst_raytrace::core::{
    Camera, Color3, DielectricMaterial, HittableList, LambertianMaterial, MetalMaterial, Point3,
    Sphere,
};

fn load_objects(world: &mut HittableList) {
    let material_ground = LambertianMaterial::new(Some(Color3::new(0.8, 0.8, 0.0)));
    let material_center = LambertianMaterial::new(Some(Color3::new(0.1, 0.2, 0.5)));
    let material_left = DielectricMaterial::new(1.5);
    let material_right = MetalMaterial::new(Some(Color3::new(0.8, 0.6, 0.2)), 0.);

    world.add(Box::new(Sphere::new(
        Point3::new(0., -100.5, -1.),
        100.,
        Box::new(material_ground),
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(0., 0., -1.),
        0.5,
        Box::new(material_center),
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(-1., 0., -1.),
        0.5,
        Box::new(material_left),
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(-1., 0., -1.),
        -0.4,
        Box::new(material_left),
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(1., 0., -1.),
        0.5,
        Box::new(material_right),
    )));
}

fn main() {
    // World
    let mut world = HittableList::new();

    load_objects(&mut world);

    let mut camera = Camera::new();

    camera.look_from = Point3::new(-2., 2., 1.);
    camera.look_at = Point3::new(0., 0., -1.);

    camera.width = 400;
    camera.aspect_ratio = 16. / 9.;
    camera.vertical_fov = 20.;

    camera.samples_per_pixel = 30;
    camera.max_ray_depth = 10;

    camera.defocus_angle = 10.;
    camera.focus_dist = 3.4;

    camera.render(&world, "out/dev-scene-1.ppm".to_owned()).err();
}
