use std::rc::Rc;

use tiny_raytracer::core::{
    get_cube_box, BoundingVolumesHierarchicalNode, CameraBuilder, Color3, EmissiveMaterial,
    HittableList, LambertianMaterial, Point3, Quad, RotateYInstance, SolidColorTexture,
    TranslateInstance, Vector3,
};

struct SceneOptions {
    bounding_volume_hierarchical: bool,
}

fn main() {
    let options = SceneOptions {
        bounding_volume_hierarchical: true,
    };

    let mut world = HittableList::new();

    // Materials
    let red = Rc::new(LambertianMaterial::new(Rc::new(
        SolidColorTexture::new_with_floats(0.65, 0.05, 0.05),
    )));
    let white = Rc::new(LambertianMaterial::new(Rc::new(
        SolidColorTexture::new_with_floats(0.73, 0.73, 0.73),
    )));
    let green = Rc::new(LambertianMaterial::new(Rc::new(
        SolidColorTexture::new_with_floats(0.12, 0.45, 0.15),
    )));
    let light = Rc::new(EmissiveMaterial::new(Rc::new(
        SolidColorTexture::new_with_floats(15., 15., 15.),
    )));

    // Primitives
    world.add(Rc::new(Quad::new(
        Point3::new(555., 0., 0.),
        Vector3::new(0., 555., 0.),
        Vector3::new(0., 0., 555.),
        green,
    )));
    world.add(Rc::new(Quad::new(
        Point3::new(0., 0., 0.),
        Vector3::new(0., 555., 0.),
        Vector3::new(0., 0., 555.),
        red,
    )));
    world.add(Rc::new(Quad::new(
        Point3::new(343., 554., 332.),
        Vector3::new(-130., 0., 0.),
        Vector3::new(0., 0., -105.),
        light,
    )));
    world.add(Rc::new(Quad::new(
        Point3::new(0., 0., 0.),
        Vector3::new(555., 0., 0.),
        Vector3::new(0., 0., 555.),
        white.clone(),
    )));
    world.add(Rc::new(Quad::new(
        Point3::new(555., 555., 555.),
        Vector3::new(-555., 0., 0.),
        Vector3::new(0., 0., -555.),
        white.clone(),
    )));
    world.add(Rc::new(Quad::new(
        Point3::new(0., 0., 555.),
        Vector3::new(555., 0., 0.),
        Vector3::new(0., 555., 0.),
        white.clone(),
    )));
    let box1 = get_cube_box(
        Point3::new(0., 0., 0.),
        Point3::new(165., 330., 165.),
        white.clone(),
    );
    let box1 = Rc::new(RotateYInstance::new(box1, 15.));
    let box1 = Rc::new(TranslateInstance::new(box1, Vector3::new(265., 0., 295.)));
    world.add(box1);
    let box2 = get_cube_box(
        Point3::new(0., 0., 0.),
        Point3::new(165., 165., 165.),
        white.clone(),
    );
    let box2 = Rc::new(RotateYInstance::new(box2, -18.));
    let box2 = Rc::new(TranslateInstance::new(box2, Vector3::new(130., 0., 65.)));
    world.add(box2);

    // BVH
    if options.bounding_volume_hierarchical {
        let bvh = BoundingVolumesHierarchicalNode::new(&mut world);
        world = HittableList::new();
        world.add(Rc::new(bvh));
    }

    // Camera
    let mut camera = CameraBuilder::default()
        .position(Point3::new(278., 278., -800.))
        .target(Point3::new(278., 278., 0.))
        .width(600)
        .aspect(1.)
        .fov(40.)
        .background(Color3::zero())
        .samples_per_pixel(128)
        .max_ray_depth(10)
        .build()
        .unwrap();
    camera
        .render(&world, "out/cornell-box-demo.ppm".to_owned())
        .err();
}
