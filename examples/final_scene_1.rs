use rand::Rng;
use rst_raytrace::core::{
    BoundingVolumesHierarchicalNode, Camera, Color3, DielectricMaterial, HittableList,
    LambertianMaterial, Material, MetalMaterial, Point3, Sphere, Vector3,
};

struct SceneOptions {
    high_quality: bool,
    depth_of_field: bool,
    motion_blur_test: bool,
    bounding_volume_hierarchical: bool,
}

fn load_objects(world: &mut HittableList, motion_blur_test: bool) {
    let material_ground = Box::new(LambertianMaterial::new(Some(Color3::new(0.5, 0.5, 0.5))));
    world.add(Box::new(Sphere::new(
        Point3::new(0., -1000., 0.),
        1000.,
        material_ground.clone(),
    )));

    let mut rng = rand::thread_rng();
    for i in -11..11 {
        for j in -11..11 {
            let choose_material = rng.gen::<f32>();
            let center = Point3::new(
                (i as f32) + 0.9 * rng.gen::<f32>(),
                0.2,
                (j as f32) + 0.9 * rng.gen::<f32>(),
            );

            if (center - &Point3::new(4., 0.2, 0.)).length() > 0.9 {
                let sphere_material: Box<dyn Material>;

                if choose_material < 0.8 {
                    // diffuse
                    let albedo =
                        Color3::random(0., 1., &mut rng) * &Color3::random(0., 1., &mut rng);
                    sphere_material = Box::new(LambertianMaterial::new(Some(albedo)));
                    world.add(Box::new(Sphere::new(center, 0.2, sphere_material.clone())));

                    if motion_blur_test {
                        let target = center + &Vector3::new(0., rng.gen_range(0.0..0.5), 0.);
                        world.add(Box::new(Sphere::new_moving_sphere(
                            center,
                            target,
                            0.2,
                            sphere_material.clone(),
                        )));
                    }
                } else if choose_material < 0.95 {
                    // metal
                    let albedo = Color3::random(0.5, 1., &mut rng);
                    let fuzz = rng.gen_range(0.0..0.5);
                    sphere_material = Box::new(MetalMaterial::new(Some(albedo), fuzz));
                    world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    // glass
                    sphere_material = Box::new(DielectricMaterial::new(1.5));
                    world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
    }

    world.add(Box::new(Sphere::new(
        Point3::new(0., 1., 0.),
        1.,
        Box::new(DielectricMaterial::new(1.5)),
    )));

    world.add(Box::new(Sphere::new(
        Point3::new(-4., 1., 0.),
        1.,
        Box::new(LambertianMaterial::new(Some(Color3::new(0.4, 0.2, 0.1)))),
    )));

    world.add(Box::new(Sphere::new(
        Point3::new(4., 1., 0.),
        1.,
        Box::new(MetalMaterial::new(Some(Color3::new(0.7, 0.6, 0.5)), 0.)),
    )));
}

fn main() {
    let options = SceneOptions {
        high_quality: false,
        depth_of_field: true,
        motion_blur_test: false,
        bounding_volume_hierarchical: true,
    };

    // World
    let mut world = HittableList::new();

    load_objects(&mut world, options.motion_blur_test);

    if options.bounding_volume_hierarchical {
        let bvh = BoundingVolumesHierarchicalNode::new(&mut world);
        world = HittableList::new();
        world.add(Box::new(bvh));
    }

    let mut camera = Camera::new();

    camera.look_from = Point3::new(13., 2., 3.);
    camera.look_at = Point3::new(0., 0., 0.);

    camera.width = if options.high_quality { 1920 } else { 400 };
    camera.aspect_ratio = 16. / 9.;
    camera.vertical_fov = 20.;

    camera.defocus_angle = if options.depth_of_field { 0.6 } else { 0.02 };
    camera.focus_dist = 10.;

    camera.samples_per_pixel = if options.high_quality { 128 } else { 20 };
    camera.max_ray_depth = 10;

    camera
        .render(&world, "out/final-scene-1.ppm".to_owned())
        .err();
}
