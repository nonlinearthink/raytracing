extern crate tiny_raytracer;

use tiny_raytracer::utils::{degree_to_radian, radian_to_degree};

#[test]
fn degree_to_radian_test() {
    assert_eq!(degree_to_radian(45.), 0.7853981633974483);
    assert_eq!(degree_to_radian(125.), 2.1816615649929116);
    assert_eq!(degree_to_radian(415.), 7.243116395776467);
}

#[test]
fn radian_to_degree_test() {
    assert_eq!(radian_to_degree(0.7853981633974483), 45.);
    assert_eq!(radian_to_degree(2.1816615649929116), 125.);
    assert_eq!(radian_to_degree(7.243116395776467), 415.);
}
