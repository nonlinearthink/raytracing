extern crate raytracing;

use raytracing::{
    core::{AxisAlignedBoundingBox, Color3, Interval, LambertianMaterial, Point3, Quad, Vector3},
    traits::Hittable,
};
use std::rc::Rc;

#[test]
fn aabb_merge_test() {
    let empty_bbox = AxisAlignedBoundingBox::default();

    let grow_bbox = AxisAlignedBoundingBox::default();

    for _ in 0..10 {
        let new_bbox = AxisAlignedBoundingBox::new(
            Interval::random(0., 10.),
            Interval::random(0., 10.),
            Interval::random(0., 10.),
        );
        let merge_empty_bbox = new_bbox.merge(&empty_bbox);
        assert_eq!(merge_empty_bbox.x, new_bbox.x);
        assert_eq!(merge_empty_bbox.y, new_bbox.y);
        assert_eq!(merge_empty_bbox.z, new_bbox.z);

        let merge_grow_bbox = new_bbox.merge(&grow_bbox);
        for axis_index in 0..3 {
            assert_eq!(
                merge_grow_bbox.axis(axis_index).min,
                f32::min(
                    grow_bbox.axis(axis_index).min,
                    new_bbox.axis(axis_index).min
                )
            );
            assert_eq!(
                merge_grow_bbox.axis(axis_index).max,
                f32::max(
                    grow_bbox.axis(axis_index).max,
                    new_bbox.axis(axis_index).max
                )
            );
        }
    }
}

#[test]
fn aabb_pad_test() {
    let quad = Quad::new(
        Point3::new(0., 0., 0.),
        Vector3::new(1., 0., 0.),
        Vector3::new(0., 1., 0.),
        Rc::new(LambertianMaterial::new_with_color(Color3::new(1., 1., 1.))),
    );
    assert!(quad.bounding_box().z.max - quad.bounding_box().z.min > 0.);
}
