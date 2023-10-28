extern crate rst_raytrace;

use rst_raytrace::core::Interval;

#[test]
fn interval_contains_test() {
    let interval = Interval::new(0., 2.);

    assert!(interval.contains(0.));
    assert!(interval.contains(1.));
    assert!(interval.contains(2.));
}

#[test]
fn interval_surrounds_test() {
    let interval = Interval::new(0., 2.);

    assert!(!interval.surrounds(0.));
    assert!(interval.surrounds(1.));
    assert!(!interval.surrounds(2.));
}

#[test]
fn interval_clamp_test() {
    let interval = Interval::new(0., 2.);

    assert_eq!(interval.clamp(-1.), 0.);
    assert_eq!(interval.clamp(1.), 1.);
    assert_eq!(interval.clamp(3.), 2.);
}
