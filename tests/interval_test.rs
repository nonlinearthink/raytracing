extern crate raytracing;

use rand::Rng;
use raytracing::core::Interval;

#[test]
fn interval_new_test() {
    let mut rng = rand::thread_rng();
    let min = rng.gen::<f32>();
    let max = rng.gen::<f32>();
    let interval = Interval::new(min, max);

    assert_eq!(min, interval.min);
    assert_eq!(max, interval.max);
}

#[test]
fn interval_empty_test() {
    let mut rng = rand::thread_rng();

    let interval = Interval::empty();

    assert!(!interval.contains(0.));
    assert!(!interval.contains(rng.gen_range(0.0..100.0)));
    assert!(!interval.contains(rng.gen_range(-100.0..0.0)));
    assert!(!interval.contains(f32::INFINITY));
    assert!(!interval.contains(f32::NEG_INFINITY));
}

#[test]
fn interval_universe_test() {
    let mut rng = rand::thread_rng();

    let interval = Interval::universe();

    assert!(interval.contains(0.));
    assert!(interval.contains(rng.gen_range(0.0..100.0)));
    assert!(interval.contains(rng.gen_range(-100.0..0.0)));
    assert!(interval.contains(f32::INFINITY));
    assert!(interval.contains(f32::NEG_INFINITY));
}

#[test]
fn interval_expand_and_size_test() {
    let mut rng = rand::thread_rng();

    let interval = Interval::new(0., 1.);

    for _ in 0..10 {
        let origin_size = interval.size();
        let increase = rng.gen_range(0.0..1.0);
        let increased_size = interval.expand(increase);
        assert!(origin_size + increase - increased_size.size() <= f32::EPSILON);
    }
}

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
