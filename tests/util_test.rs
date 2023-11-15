extern crate tiny_raytracer;

use tiny_raytracer::utils::{deg_to_rad, rad_to_deg};

#[test]
fn deg_to_rad_test() {
    assert_eq!(deg_to_rad(45.), 0.7853981633974483);
    assert_eq!(deg_to_rad(125.), 2.1816615649929116);
    assert_eq!(deg_to_rad(415.), 7.243116395776467);
}

#[test]
fn rad_to_deg_test() {
    assert_eq!(rad_to_deg(0.7853981633974483), 45.);
    assert_eq!(rad_to_deg(2.1816615649929116), 125.);
    assert_eq!(rad_to_deg(7.243116395776467), 415.);
}
