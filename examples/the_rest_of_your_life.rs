use raytracing::{
    core::{
        get_cube_box, BVHNode, CameraBuilder, Color3, ConstantMedium, DielectricMaterial,
        EmissiveMaterial, HittableList, LambertianMaterial, MetalMaterial, Point3, Quad,
        RotateYInstance, SolidColorTexture, Sphere, TranslateInstance, Vector3,
    },
    traits::Hittable,
};
use std::rc::Rc;

struct SceneOptions {
    bounding_volume_hierarchical: bool,
    smoke_test: bool,
    metal_test: bool,
    replace_box_with_sphere: bool,
    high_quality: bool,
}

fn main() {
    let options = SceneOptions {
        bounding_volume_hierarchical: false,
        // FIXME: right side of box is gone
        smoke_test: false,
        metal_test: false,
        replace_box_with_sphere: true,
        high_quality: false,
    };

    let mut world = HittableList::new();
    let mut lights = HittableList::new();

    // Materials
    let red = Rc::new(LambertianMaterial::new(Rc::new(SolidColorTexture::new(
        0.65, 0.05, 0.05,
    ))));
    let white = Rc::new(LambertianMaterial::new(Rc::new(SolidColorTexture::new(
        0.73, 0.73, 0.73,
    ))));
    let green = Rc::new(LambertianMaterial::new(Rc::new(SolidColorTexture::new(
        0.12, 0.45, 0.15,
    ))));
    let light = Rc::new(EmissiveMaterial::new(Rc::new(if options.smoke_test {
        SolidColorTexture::new(7., 7., 7.)
    } else {
        SolidColorTexture::new(15., 15., 15.)
    })));
    let metal = Rc::new(MetalMaterial::new(
        Rc::new(SolidColorTexture::new(0.8, 0.85, 0.88)),
        0.,
    ));
    let glass = Rc::new(DielectricMaterial::new(1.5));

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
    if options.smoke_test {
        let light_source = Rc::new(Quad::new(
            Point3::new(113., 554., 127.),
            Vector3::new(330., 0., 0.),
            Vector3::new(0., 0., 305.),
            light,
        ));
        lights.add(light_source.clone());
        world.add(light_source);
        world.add(Rc::new(Quad::new(
            Point3::new(0., 555., 0.),
            Vector3::new(555., 0., 0.),
            Vector3::new(0., 0., 555.),
            white.clone(),
        )));
    } else {
        let light_source = Rc::new(Quad::new(
            Point3::new(343., 554., 332.),
            Vector3::new(-130., 0., 0.),
            Vector3::new(0., 0., -105.),
            light,
        ));
        lights.add(light_source.clone());
        world.add(light_source);
        world.add(Rc::new(Quad::new(
            Point3::new(555., 555., 555.),
            Vector3::new(-555., 0., 0.),
            Vector3::new(0., 0., -555.),
            white.clone(),
        )));
    }
    world.add(Rc::new(Quad::new(
        Point3::new(0., 0., 0.),
        Vector3::new(555., 0., 0.),
        Vector3::new(0., 0., 555.),
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
        if options.metal_test {
            metal
        } else {
            white.clone()
        },
    );
    let box1 = Rc::new(RotateYInstance::new(box1, 15.));
    let box1 = Rc::new(TranslateInstance::new(box1, Vector3::new(265., 0., 295.)));
    if options.smoke_test {
        world.add(Rc::new(ConstantMedium::new_with_color(
            box1,
            0.01,
            Color3::new(0., 0., 0.),
        )));
    } else {
        world.add(box1);
    }

    if options.replace_box_with_sphere {
        let sphere = Rc::new(Sphere::new(Point3::new(190., 90., 190.), 90., glass));
        // lights.add(sphere.clone());
        world.add(sphere);
    } else {
        let box2 = get_cube_box(
            Point3::new(0., 0., 0.),
            Point3::new(165., 165., 165.),
            white.clone(),
        );
        let box2 = Rc::new(RotateYInstance::new(box2, -18.));
        let box2 = Rc::new(TranslateInstance::new(box2, Vector3::new(130., 0., 65.)));
        if options.smoke_test {
            world.add(Rc::new(ConstantMedium::new_with_color(
                box2,
                0.01,
                Color3::new(1., 1., 1.),
            )));
        } else {
            world.add(box2);
        }
    }

    // BVH
    if options.bounding_volume_hierarchical {
        let bvh = BVHNode::new(&mut world);
        world = HittableList::new();
        world.add(Rc::new(bvh));
    }

    // Camera
    let mut camera = CameraBuilder::default()
        .position(Point3::new(278., 278., -800.))
        .target(Point3::new(278., 278., 0.))
        .width(if options.high_quality { 1200 } else { 600 })
        .aspect(1.)
        .fov(40.)
        .background(Color3::zero())
        .samples_per_pixel(if options.high_quality { 1000 } else { 128 })
        .max_ray_depth(50)
        .build()
        .unwrap();
    let world = Rc::new(world);
    let lights: Option<Rc<dyn Hittable>> = Some(Rc::new(lights));
    camera
        .render(world, lights, "out/the-rest-of-your-life.ppm".to_owned())
        .err();
}
