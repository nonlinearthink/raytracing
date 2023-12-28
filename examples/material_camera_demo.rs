use std::rc::Rc;

use raytracing::core::{
    BVHNode, CameraBuilder, Color3, DielectricMaterial, HittableList, LambertianMaterial,
    MetalMaterial, Point3, SolidColorTexture, Sphere,
};

struct SceneOptions {
    bounding_volume_hierarchical: bool,
    larger_fov: bool,
    depth_of_field: bool,
}

fn main() {
    let options = SceneOptions {
        bounding_volume_hierarchical: true,
        larger_fov: false,
        depth_of_field: false,
    };

    let mut world = HittableList::new();

    // Materials
    let material_ground = Rc::new(LambertianMaterial::new(Rc::new(SolidColorTexture::new(
        0.8, 0.8, 0.,
    ))));
    let material_center = Rc::new(LambertianMaterial::new(Rc::new(SolidColorTexture::new(
        0.1, 0.2, 0.5,
    ))));
    let material_left = Rc::new(DielectricMaterial::new(1.5));
    let material_right = Rc::new(MetalMaterial::new(
        Rc::new(SolidColorTexture::new(0.8, 0.6, 0.2)),
        0.,
    ));

    // Primitives
    world.add(Rc::new(Sphere::new(
        Point3::new(0., -100.5, -1.),
        100.,
        material_ground.clone(),
    )));
    world.add(Rc::new(Sphere::new(
        Point3::new(0., 0., -1.),
        0.5,
        material_center.clone(),
    )));
    world.add(Rc::new(Sphere::new(
        Point3::new(-1., 0., -1.),
        0.5,
        material_left.clone(),
    )));
    world.add(Rc::new(Sphere::new(
        Point3::new(-1., 0., -1.),
        -0.4,
        material_left.clone(),
    )));
    world.add(Rc::new(Sphere::new(
        Point3::new(1., 0., -1.),
        0.5,
        material_right.clone(),
    )));

    // BVH
    if options.bounding_volume_hierarchical {
        let bvh = BVHNode::new(&mut world);
        world = HittableList::new();
        world.add(Rc::new(bvh));
    }

    // Camera
    let mut camera_builder = CameraBuilder::default();
    let mut camera_builder_mut_ref = camera_builder
        .position(Point3::new(-2., 2., 1.))
        .target(Point3::new(0., 0., -1.))
        .width(400)
        .aspect(16. / 9.)
        .fov(if options.larger_fov { 90. } else { 20. })
        .background(Color3::new(0.7, 0.8, 1.))
        .samples_per_pixel(30)
        .max_ray_depth(10);
    if options.depth_of_field {
        camera_builder_mut_ref = camera_builder_mut_ref.defocus_angle(10.).focus_dist(3.4);
    }
    let mut camera = camera_builder_mut_ref.build().unwrap();
    let world = Rc::new(world);
    camera
        .render(world, None, "out/material-camera-demo.ppm".to_owned())
        .err();
}
