use tiny_raytracer::core::{
    BoundingVolumesHierarchicalNode, Camera, CheckerTexture, Color3, HittableList,
    LambertianMaterial, Point3, Sphere,
};

struct SceneOptions {
    bounding_volume_hierarchical: bool,
}

fn main() {
    let options = SceneOptions {
        bounding_volume_hierarchical: false,
    };

    let mut world = HittableList::new();

    let checker_texture = Box::new(CheckerTexture::new_with_solid_color(
        0.32,
        Color3::new(0.2, 0.3, 0.1),
        Color3::new(0.9, 0.9, 0.9),
    ));

    let material = Box::new(LambertianMaterial::new(checker_texture));

    world.add(Box::new(Sphere::new(
        Point3::new(0., -10., 0.),
        10.,
        material.clone(),
    )));

    world.add(Box::new(Sphere::new(
        Point3::new(0., 10., 0.),
        10.,
        material.clone(),
    )));

    if options.bounding_volume_hierarchical {
        let bvh = BoundingVolumesHierarchicalNode::new(&mut world);
        world = HittableList::new();
        world.add(Box::new(bvh));
    }

    let mut camera = Camera::new();

    camera.look_from = Point3::new(13., 2., 3.);
    camera.look_at = Point3::new(0., 0., 0.);

    camera.width = 400;
    camera.aspect_ratio = 16. / 9.;
    camera.vertical_fov = 20.;

    camera.focus_dist = 10.;

    camera.samples_per_pixel = 30;
    camera.max_ray_depth = 10;

    camera
        .render(&world, "out/checker-texture-demo.ppm".to_owned())
        .err();
}
