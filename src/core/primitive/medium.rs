use std::rc::Rc;

use crate::core::{
    Color3, HitRecord, Hittable, Interval, IsotropicMaterial, Material, Texture, Vector3,
};

#[derive(Debug)]
pub struct ConstantMedium {
    boundary: Rc<dyn Hittable>,
    neg_inv_density: f32,
    phase_function: Rc<dyn Material>,
}

impl ConstantMedium {
    pub fn new(boundary: Rc<dyn Hittable>, density: f32, albedo: Rc<dyn Texture>) -> Self {
        Self {
            boundary,
            neg_inv_density: -1. / density,
            phase_function: Rc::new(IsotropicMaterial::new(albedo)),
        }
    }

    pub fn new_with_color(boundary: Rc<dyn Hittable>, density: f32, albedo_color: Color3) -> Self {
        Self {
            boundary,
            neg_inv_density: -1. / density,
            phase_function: Rc::new(IsotropicMaterial::new_with_color(albedo_color)),
        }
    }
}

impl Hittable for ConstantMedium {
    fn hit(
        &self,
        ray: &crate::core::Ray,
        ray_interval: &crate::core::Interval,
        record: &mut crate::core::HitRecord,
    ) -> bool {
        let enable_debug = false;
        let debugging = enable_debug && rand::random::<f32>() < 0.00001;

        let (mut rec1, mut rec2) = (HitRecord::new(), HitRecord::new());

        if !self.boundary.hit(ray, &Interval::universe(), &mut rec1) {
            return false;
        }

        if !self.boundary.hit(
            ray,
            &Interval::new(rec1.t + 0.0001, f32::INFINITY),
            &mut rec2,
        ) {
            return false;
        }

        if debugging {
            print!("\nray_tmin={}, ray_tmax={}\n", rec1.t, rec2.t);
        }

        if rec1.t < ray_interval.min {
            rec1.t = ray_interval.min;
        }
        if rec2.t > ray_interval.max {
            rec2.t = ray_interval.max;
        }
        if rec1.t >= rec2.t {
            return false;
        }

        if rec1.t < 0. {
            rec1.t = 0.;
        }

        let ray_length = ray.direction.length();
        let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
        let hit_distance = self.neg_inv_density * f32::log2(rand::random::<f32>());

        if hit_distance > distance_inside_boundary {
            return false;
        }

        record.t = rec1.t + hit_distance / ray_length;
        record.point = Some(ray.at(record.t));

        if debugging {
            print!(
                "hit_distance = {}\nrec.t = {}\nrec.point = {:?}\n",
                hit_distance, record.t, record.point
            );
        }

        record.normal = Some(Vector3::new(1., 0., 0.)); // arbitrary
        record.front_face = true;
        record.material = Some(Rc::clone(&self.phase_function));

        true
    }

    fn bounding_box(&self) -> &crate::core::AxisAlignedBoundingBox {
        self.boundary.bounding_box()
    }
}
