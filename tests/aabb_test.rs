extern crate rst_raytrace;

use rst_raytrace::core::{AxisAlignedBoundingBox, Interval, Ray, Vector3};

#[test]
fn aabb_merge_test() {
    let empty_aabb = AxisAlignedBoundingBox::default();

    let grow_aabb = AxisAlignedBoundingBox::default();

    for _ in 0..10 {
        let new_aabb = AxisAlignedBoundingBox::new(
            Interval::random(0., 10.),
            Interval::random(0., 10.),
            Interval::random(0., 10.),
        );
        let merge_empty_aabb = new_aabb.merge(&empty_aabb);
        assert_eq!(merge_empty_aabb.x, new_aabb.x);
        assert_eq!(merge_empty_aabb.y, new_aabb.y);
        assert_eq!(merge_empty_aabb.y, new_aabb.y);

        let merge_grow_aabb = new_aabb.merge(&grow_aabb);
        for axis_index in 0..3 {
            assert_eq!(
                merge_grow_aabb.axis(axis_index).min,
                f32::min(
                    grow_aabb.axis(axis_index).min,
                    new_aabb.axis(axis_index).min
                )
            );
            assert_eq!(
                merge_grow_aabb.axis(axis_index).max,
                f32::max(
                    grow_aabb.axis(axis_index).max,
                    new_aabb.axis(axis_index).max
                )
            );
        }
    }
}

#[test]
fn aabb_hit_test() {
    let ray = Ray::new(Vector3::zero(), Vector3::one());

    let aabb = AxisAlignedBoundingBox::from_bounding_vector(
        &Vector3::new(2., 2., 2.),
        &Vector3::new(3., 3., 3.),
    );

    assert!(!aabb.hit(&ray, &mut Interval::new(0., 1.999)));
    assert!(aabb.hit(&ray, &mut Interval::new(0., 2.)));
    assert!(!aabb.hit(&ray, &mut Interval::new(0., f32::NEG_INFINITY)));
    assert!(aabb.hit(&ray, &mut Interval::new(0., f32::INFINITY)));
}
