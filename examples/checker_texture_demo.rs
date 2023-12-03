use std::rc::Rc;

use raytracing::core::{
    BVHNode, CameraBuilder, CheckerTexture, Color3, HittableList, LambertianMaterial, Point3,
    Sphere,
};

struct SceneOptions {
    bounding_volume_hierarchical: bool,
}

fn main() {
    let options = SceneOptions {
        bounding_volume_hierarchical: false,
    };

    let mut world = HittableList::new();

    // Textures
    let checker_texture = Rc::new(CheckerTexture::new_with_solid_color(
        0.32,
        Color3::new(0.2, 0.3, 0.1),
        Color3::new(0.9, 0.9, 0.9),
    ));

    // Materials
    let material = Rc::new(LambertianMaterial::new(checker_texture));

    // Primitives
    world.add(Rc::new(Sphere::new(
        Point3::new(0., -10., 0.),
        10.,
        material.clone(),
    )));
    world.add(Rc::new(Sphere::new(
        Point3::new(0., 10., 0.),
        10.,
        material.clone(),
    )));

    // BVH
    if options.bounding_volume_hierarchical {
        let bvh = BVHNode::new(&mut world);
        world = HittableList::new();
        world.add(Rc::new(bvh));
    }

    // Camera
    let mut camera = CameraBuilder::default()
        .position(Point3::new(13., 2., 3.))
        .target(Point3::zero())
        .width(400)
        .aspect(16. / 9.)
        .fov(20.)
        .focus_dist(10.)
        .background(Color3::new(0.7, 0.8, 1.))
        .samples_per_pixel(30)
        .max_ray_depth(10)
        .build()
        .unwrap();
    camera
        .render(&world, "out/checker-texture-demo.ppm".to_owned())
        .err();
}
