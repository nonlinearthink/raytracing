use rand::random;

fn uniform_f(d: f32) -> f32 {
    2. * d
}

fn uniform_pdf(_x: f32) -> f32 {
    0.5
}

fn linear_f(d: f32) -> f32 {
    f32::sqrt(4.0 * d)
}

fn linear_pdf(x: f32) -> f32 {
    x / 2.0
}

fn quadratic_f(d: f32) -> f32 {
    8. * d.powf(1. / 3.)
}

fn quadratic_pdf(x: f32) -> f32 {
    (3. / 8.) * x * x
}

fn main() {
    let n = 1000000;
    let mut sum = 0.0;
    for _ in 0..n {
        let x = uniform_f(random());
        sum += x * x / uniform_pdf(x);
    }
    println!("Uniform PDF: I = {}", sum / n as f32);

    let n = 1000000;
    let mut sum = 0.0;
    for _ in 0..n {
        let x = linear_f(random());
        sum += x * x / linear_pdf(x);
    }
    println!("Linear PDF: I = {}", sum / n as f32);

    let n = 1;
    let mut sum = 0.0;
    for _ in 0..n {
        let x = quadratic_f(random());
        sum += x * x / quadratic_pdf(x);
    }
    println!("Quadratic PDF: I = {}", sum / n as f32);
}
