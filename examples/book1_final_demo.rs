use rand::Rng;
use std::rc::Rc;

use tiny_raytracer::core::{
    BoundingVolumesHierarchicalNode, Camera, CheckerTexture, Color3, DielectricMaterial,
    HittableList, LambertianMaterial, Material, MetalMaterial, Point3, SolidColorTexture, Sphere,
    Texture, Vector3,
};

struct SceneOptions {
    depth_of_field: bool,
    high_quality: bool,
    motion_blur_test: bool,
    bounding_volume_hierarchical: bool,
    checker_texture_test: bool,
}

fn load_objects(world: &mut HittableList, motion_blur_test: bool, checker_texture_test: bool) {
    let ground_texture: Rc<dyn Texture> = if checker_texture_test {
        Rc::new(CheckerTexture::new_with_solid_color(
            0.32,
            Color3::new(0.2, 0.3, 0.1),
            Color3::new(0.9, 0.9, 0.9),
        ))
    } else {
        Rc::new(SolidColorTexture::new_with_floats(0.5, 0.5, 0.5))
    };
    let material_ground = Rc::new(LambertianMaterial::new(ground_texture));
    world.add(Rc::new(Sphere::new(
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
                let sphere_material: Rc<dyn Material>;

                if choose_material < 0.8 {
                    // diffuse
                    let albedo_texture = Rc::new(SolidColorTexture::new(
                        Color3::random(0., 1., &mut rng) * &Color3::random(0., 1., &mut rng),
                    ));
                    sphere_material = Rc::new(LambertianMaterial::new(albedo_texture));
                    world.add(Rc::new(Sphere::new(center, 0.2, sphere_material.clone())));

                    if motion_blur_test {
                        let target = center + &Vector3::new(0., rng.gen_range(0.0..0.5), 0.);
                        world.add(Rc::new(Sphere::new_moving_sphere(
                            center,
                            target,
                            0.2,
                            sphere_material.clone(),
                        )));
                    }
                } else if choose_material < 0.95 {
                    // metal
                    let albedo_texture =
                        Rc::new(SolidColorTexture::new(Color3::random(0.5, 1., &mut rng)));
                    let fuzz = rng.gen_range(0.0..0.5);
                    sphere_material = Rc::new(MetalMaterial::new(albedo_texture, fuzz));
                    world.add(Rc::new(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    // glass
                    sphere_material = Rc::new(DielectricMaterial::new(1.5));
                    world.add(Rc::new(Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
    }

    world.add(Rc::new(Sphere::new(
        Point3::new(0., 1., 0.),
        1.,
        Rc::new(DielectricMaterial::new(1.5)),
    )));

    world.add(Rc::new(Sphere::new(
        Point3::new(-4., 1., 0.),
        1.,
        Rc::new(LambertianMaterial::new(Rc::new(
            SolidColorTexture::new_with_floats(0.4, 0.2, 0.1),
        ))),
    )));

    world.add(Rc::new(Sphere::new(
        Point3::new(4., 1., 0.),
        1.,
        Rc::new(MetalMaterial::new(
            Rc::new(SolidColorTexture::new_with_floats(0.7, 0.6, 0.5)),
            0.,
        )),
    )));
}

fn main() {
    let options = SceneOptions {
        depth_of_field: true,
        high_quality: false,
        motion_blur_test: false,
        bounding_volume_hierarchical: true,
        checker_texture_test: false,
    };

    // World
    let mut world = HittableList::new();

    load_objects(
        &mut world,
        options.motion_blur_test,
        options.checker_texture_test,
    );

    if options.bounding_volume_hierarchical {
        let bvh = BoundingVolumesHierarchicalNode::new(&mut world);
        world = HittableList::new();
        world.add(Rc::new(bvh));
    }

    let mut camera = Camera::new();

    camera.look_from = Point3::new(13., 2., 3.);
    camera.look_at = Point3::new(0., 0., 0.);

    camera.width = if options.high_quality { 1920 } else { 400 };
    camera.aspect_ratio = 16. / 9.;
    camera.vertical_fov = 20.;

    camera.defocus_angle = if options.depth_of_field { 0.6 } else { 0.02 };
    camera.focus_dist = 10.;

    camera.samples_per_pixel = if options.high_quality { 128 } else { 30 };
    camera.max_ray_depth = 10;

    camera
        .render(&world, "out/book1-final-demo.ppm".to_owned())
        .err();
}
