extern crate raytracing;

use raytracing::core::{AxisAlignedBoundingBox, Interval, Ray, Vector3};

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
fn aabb_hit_test() {
    let ray = Ray::new(Vector3::zero(), Vector3::one());

    let bbox = AxisAlignedBoundingBox::from_bounding_points(
        &Vector3::new(2., 2., 2.),
        &Vector3::new(3., 3., 3.),
    );

    assert!(!bbox.hit(&ray, &mut Interval::new(0., 1.999)));
    assert!(bbox.hit(&ray, &mut Interval::new(0., 2.)));
    assert!(!bbox.hit(&ray, &mut Interval::new(0., f32::NEG_INFINITY)));
    assert!(bbox.hit(&ray, &mut Interval::new(0., f32::INFINITY)));
}
