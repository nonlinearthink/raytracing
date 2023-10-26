use rst_raytrace::core::Interval;

extern crate rst_raytrace;

#[test]
fn interval_contains_test() {
    let interval = Interval::new(0., 2.);

    assert!(interval.contains(0.));
    assert!(interval.contains(1.));
    assert!(interval.contains(2.));
}

#[test]
fn interval_test() {
    let interval = Interval::new(0., 2.);

    assert!(!interval.surrounds(0.));
    assert!(interval.surrounds(1.));
    assert!(!interval.surrounds(2.));
}
