use std::rc::Rc;

use tiny_raytracer::core::{
    BoundingVolumesHierarchicalNode, Camera, DielectricMaterial, HittableList, LambertianMaterial,
    MetalMaterial, Point3, SolidColorTexture, Sphere,
};

struct SceneOptions {
    bounding_volume_hierarchical: bool,
    larger_fov: bool,
    depth_of_field: bool,
}

fn load_objects(world: &mut HittableList) {
    let material_ground = Rc::new(LambertianMaterial::new(Rc::new(
        SolidColorTexture::new_with_floats(0.8, 0.8, 0.),
    )));
    let material_center = Rc::new(LambertianMaterial::new(Rc::new(
        SolidColorTexture::new_with_floats(0.1, 0.2, 0.5),
    )));
    let material_left = Rc::new(DielectricMaterial::new(1.5));
    let material_right = Rc::new(MetalMaterial::new(
        Rc::new(SolidColorTexture::new_with_floats(0.8, 0.6, 0.2)),
        0.,
    ));

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
}

fn main() {
    let options = SceneOptions {
        bounding_volume_hierarchical: false,
        larger_fov: false,
        depth_of_field: false,
    };

    // World
    let mut world = HittableList::new();

    load_objects(&mut world);

    if options.bounding_volume_hierarchical {
        let bvh = BoundingVolumesHierarchicalNode::new(&mut world);
        world = HittableList::new();
        world.add(Rc::new(bvh));
    }

    let mut camera = Camera::new();

    camera.look_from = Point3::new(-2., 2., 1.);
    camera.look_at = Point3::new(0., 0., -1.);

    camera.width = 400;
    camera.aspect_ratio = 16. / 9.;
    camera.vertical_fov = if options.larger_fov { 90. } else { 20. };

    camera.samples_per_pixel = 30;
    camera.max_ray_depth = 10;

    if options.depth_of_field {
        camera.defocus_angle = 10.;
        camera.focus_dist = 3.4;
    }

    camera
        .render(&world, "out/material-camera-demo.ppm".to_owned())
        .err();
}
