use rand::Rng;

fn main() {
    let sqrt_n = 1000;
    let mut rng = rand::thread_rng();

    let mut inside_count = 0;
    let mut inside_circle_stratified = 0;

    for i in 0..sqrt_n {
        for j in 0..sqrt_n {
            let x = rng.gen_range(-1.0..1.0);
            let y = rng.gen_range(-1.0..1.0);
            if x * x + y * y < 1. {
                inside_count += 1;
            }
            let x = 2. * ((i as f64 + rng.gen::<f64>()) / sqrt_n as f64) - 1.;
            let y = 2. * ((j as f64 + rng.gen::<f64>()) / sqrt_n as f64) - 1.;
            if x * x + y * y < 1. {
                inside_circle_stratified += 1;
            }
        }
    }

    // area(circle) / area(square) == π / 4
    println!(
        "Regular Estimate of Pi = {}",
        (4. * inside_count as f64) / (sqrt_n * sqrt_n) as f64
    );
    println!(
        "Stratified Estimate of Pi = {}",
        (4. * inside_circle_stratified as f64) / (sqrt_n * sqrt_n) as f64
    );
}
