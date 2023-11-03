use rand::Rng;

use rst_raytrace::core::{
    Camera, Color3, DielectricMaterial, HittableList, LambertianMaterial, Material, MetalMaterial,
    Point3, Sphere, Vector3,
};

fn load_objects_1_13(world: &mut HittableList) {
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

fn load_objects_1_14(world: &mut HittableList, motion_blur_test: bool) {
    let material_ground = LambertianMaterial::new(Some(Color3::new(0.5, 0.5, 0.5)));
    world.add(Box::new(Sphere::new(
        Point3::new(0., -1000., 0.),
        1000.,
        Box::new(material_ground),
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
                    // Chapter 2-2
                    if motion_blur_test {
                        let target = center + &Vector3::new(0., rng.gen_range(0.0..0.3), 0.);
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

    let material1 = DielectricMaterial::new(1.5);
    world.add(Box::new(Sphere::new(
        Point3::new(0., 1., 0.),
        1.,
        Box::new(material1),
    )));

    let material2 = LambertianMaterial::new(Some(Color3::new(0.4, 0.2, 0.1)));
    world.add(Box::new(Sphere::new(
        Point3::new(-4., 1., 0.),
        1.,
        Box::new(material2),
    )));

    let material3 = MetalMaterial::new(Some(Color3::new(0.7, 0.6, 0.5)), 0.);
    world.add(Box::new(Sphere::new(
        Point3::new(4., 1., 0.),
        1.,
        Box::new(material3),
    )));
}

pub fn example_1_13() {
    // World
    let mut world = HittableList::new();

    load_objects_1_13(&mut world);

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

    camera.render(&world, "out/1-13.ppm".to_owned()).err();
}

// Book 1 Final Scene
pub fn example_1_14() {
    // World
    let mut world = HittableList::new();

    load_objects_1_14(&mut world, false);

    let mut camera = Camera::new();

    camera.look_from = Point3::new(13., 2., 3.);
    camera.look_at = Point3::new(0., 0., 0.);

    camera.width = 400;
    camera.aspect_ratio = 16. / 9.;
    camera.vertical_fov = 20.;

    camera.defocus_angle = 0.6;
    camera.focus_dist = 10.;

    camera.samples_per_pixel = 50;
    camera.max_ray_depth = 10;

    camera.render(&world, "out/1-14.ppm".to_owned()).err();
}

pub fn example_2_2() {
    // World
    let mut world = HittableList::new();

    load_objects_1_14(&mut world, true);

    let mut camera = Camera::new();

    camera.look_from = Point3::new(13., 2., 3.);
    camera.look_at = Point3::new(0., 0., 0.);

    camera.width = 400;
    camera.aspect_ratio = 16. / 9.;
    camera.vertical_fov = 20.;

    camera.defocus_angle = 0.02;
    camera.focus_dist = 10.;

    camera.samples_per_pixel = 128;
    camera.max_ray_depth = 10;

    camera.render(&world, "out/2-2.ppm".to_owned()).err();
}
