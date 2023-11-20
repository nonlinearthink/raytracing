use rand::{rngs::ThreadRng, Rng};
use std::rc::Rc;
use tiny_raytracer::core::{
    get_cube_box, BoundingVolumesHierarchicalNode, CameraBuilder, Color3, ConstantMedium,
    DielectricMaterial, EmissiveMaterial, HittableList, ImageTexture, LambertianMaterial,
    MetalMaterial, NoiseTexture, Point3, Quad, RotateYInstance, SolidColorTexture, Sphere,
    TranslateInstance, Vector3,
};

const BOXES_PER_SIDE: u32 = 20;

struct SceneOptions {
    high_quality: bool,
}

fn load_ground(world: &mut HittableList, rng: &mut ThreadRng) {
    let mut box_list1 = HittableList::new();
    let ground_material = Rc::new(LambertianMaterial::new_with_color(Color3::new(
        0.48, 0.83, 0.53,
    )));
    for i in 0..BOXES_PER_SIDE {
        for j in 0..BOXES_PER_SIDE {
            let w = 100.0;
            let x0 = -1000.0 + w * i as f32;
            let z0 = -1000.0 + w * j as f32;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = rng.gen_range(1.0..101.0);
            let z1 = z0 + w;

            box_list1.add(get_cube_box(
                Point3::new(x0, y0, z0),
                Point3::new(x1, y1, z1),
                ground_material.clone(),
            ));
        }
    }
    world.add(Rc::new(BoundingVolumesHierarchicalNode::new(
        &mut box_list1,
    )));
}

fn load_light(world: &mut HittableList) {
    let light_material = Rc::new(EmissiveMaterial::new_with_color(Color3::new(7., 7., 7.)));
    world.add(Rc::new(Quad::new(
        Point3::new(123., 554., 147.),
        Vector3::new(300., 0., 0.),
        Vector3::new(0., 0., 265.),
        light_material,
    )));
}

fn load_moving_sphere(world: &mut HittableList) {
    let center1 = Point3::new(400., 400., 200.);
    let center2 = &center1 + &Vector3::new(30., 0., 0.);
    let sphere_material = Rc::new(LambertianMaterial::new_with_color(Color3::new(
        0.7, 0.3, 0.1,
    )));
    world.add(Rc::new(Sphere::new_moving_sphere(
        center1,
        center2,
        50.,
        sphere_material,
    )));
}

fn load_sphere_with_materials(world: &mut HittableList) {
    // Dielectric
    world.add(Rc::new(Sphere::new(
        Point3::new(260., 150., 45.),
        50.,
        Rc::new(DielectricMaterial::new(1.5)),
    )));

    // Metal
    world.add(Rc::new(Sphere::new(
        Point3::new(0., 150., 145.),
        50.,
        Rc::new(MetalMaterial::new(
            Rc::new(SolidColorTexture::new(Color3::new(0.8, 0.8, 0.9))),
            1.0,
        )),
    )));
}

fn load_volume(world: &mut HittableList) {
    let boundary = Rc::new(Sphere::new(
        Point3::new(360., 150., 145.),
        70.,
        Rc::new(DielectricMaterial::new(1.5)),
    ));
    world.add(boundary.clone());
    world.add(Rc::new(ConstantMedium::new_with_color(
        boundary,
        0.2,
        Color3::new(0.2, 0.4, 0.9),
    )));
    let boundary = Rc::new(Sphere::new(
        Point3::new(0., 0., 0.),
        5000.,
        Rc::new(DielectricMaterial::new(1.5)),
    ));
    world.add(Rc::new(ConstantMedium::new_with_color(
        boundary,
        0.0001,
        Color3::new(1., 1., 1.),
    )));
}

fn load_earth(world: &mut HittableList) {
    let earth_material = Rc::new(LambertianMaterial::new(Rc::new(
        ImageTexture::new("assets/earthmap.jpg".to_owned()).unwrap(),
    )));
    world.add(Rc::new(Sphere::new(
        Point3::new(400., 200., 400.),
        100.,
        earth_material,
    )));
}

fn load_perlin_noise(world: &mut HittableList) {
    let noise_texture = Rc::new(NoiseTexture::new_with_marble_effect(0.1));
    world.add(Rc::new(Sphere::new(
        Point3::new(220., 280., 300.),
        80.,
        Rc::new(LambertianMaterial::new(noise_texture)),
    )));
}

fn load_box_cluster(world: &mut HittableList, rng: &mut ThreadRng) {
    let mut box_list2 = HittableList::new();
    let white_material = Rc::new(LambertianMaterial::new_with_color(Color3::new(
        0.73, 0.73, 0.73,
    )));
    let ns = 1000;
    for _ in 0..ns {
        box_list2.add(Rc::new(Sphere::new(
            Point3::random(0., 165., rng),
            10.,
            white_material.clone(),
        )));
    }
    world.add(Rc::new(TranslateInstance::new(
        Rc::new(RotateYInstance::new(
            Rc::new(BoundingVolumesHierarchicalNode::new(&mut box_list2)),
            15.,
        )),
        Vector3::new(-100., 270., 395.),
    )));
}

fn load_primitives(world: &mut HittableList) {
    let mut rng = rand::thread_rng();

    load_ground(world, &mut rng);

    load_light(world);

    load_moving_sphere(world);

    load_sphere_with_materials(world);

    load_volume(world);

    load_earth(world);

    load_perlin_noise(world);

    load_box_cluster(world, &mut rng);
}

fn main() {
    let options = SceneOptions { high_quality: false };

    // World
    let mut world = HittableList::new();

    // Primitives
    load_primitives(&mut world);

    // Camera
    let mut camera = CameraBuilder::default()
        .position(Point3::new(478., 278., -600.))
        .target(Point3::new(278., 278., 0.))
        .width(if options.high_quality { 800 } else { 400 })
        .aspect(1.)
        .fov(40.)
        .background(Color3::zero())
        .samples_per_pixel(if options.high_quality { 1000 } else { 128 })
        .max_ray_depth(10)
        .build()
        .unwrap();
    camera
        .render(&world, "out/book2-final-demo.ppm".to_owned())
        .err();
}
