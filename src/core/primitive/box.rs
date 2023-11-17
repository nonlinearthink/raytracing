use super::Quad;
use crate::core::{Hittable, HittableList, Material, Point3, Vector3};
use std::rc::Rc;

pub fn get_cube_box(start: Point3, end: Point3, material: Rc<dyn Material>) -> Rc<dyn Hittable> {
    let mut cube_box = Rc::new(HittableList::new());
    let cube_box_mut_ref = Rc::get_mut(&mut cube_box).unwrap();

    let min = Point3::new(
        f32::min(start.x, end.x),
        f32::min(start.y, end.y),
        f32::min(start.z, end.z),
    );
    let max = Point3::new(
        f32::max(start.x, end.x),
        f32::max(start.y, end.y),
        f32::max(start.z, end.z),
    );

    let dx = Vector3::new(max.x - min.x, 0., 0.);
    let dy = Vector3::new(0., max.y - min.y, 0.);
    let dz = Vector3::new(0., 0., max.z - min.z);

    cube_box_mut_ref.add(Rc::new(Quad::new(
        Point3::new(min.x, min.y, max.z),
        dx,
        dy,
        material.clone(),
    ))); // front
    cube_box_mut_ref.add(Rc::new(Quad::new(
        Point3::new(max.x, min.y, max.z),
        -dz,
        dy,
        material.clone(),
    ))); // right
    cube_box_mut_ref.add(Rc::new(Quad::new(
        Point3::new(max.x, min.y, min.z),
        -dx,
        dy,
        material.clone(),
    ))); // back
    cube_box_mut_ref.add(Rc::new(Quad::new(
        Point3::new(min.x, min.y, min.z),
        dx,
        dy,
        material.clone(),
    ))); // left
    cube_box_mut_ref.add(Rc::new(Quad::new(
        Point3::new(min.x, max.y, max.z),
        dx,
        -dz,
        material.clone(),
    ))); // top
    cube_box_mut_ref.add(Rc::new(Quad::new(
        Point3::new(min.x, min.y, min.z),
        dx,
        dz,
        material.clone(),
    ))); // bottom

    cube_box
}
