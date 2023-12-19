use raytracing::core::Vector3;

fn f(d: &Vector3) -> f32 {
    d.z * d.z
}

fn pdf(_d: &Vector3) -> f32 {
    1. / (4. * std::f32::consts::PI)
}

fn main() {
    let n = 1000000;
    let mut sum = 0.0;
    for _ in 0..n {
        let d = Vector3::random_unit_vector();
        sum += f(&d) / pdf(&d);
    }
    println!("Uniform PDF: I = {}", sum / n as f32);
}
