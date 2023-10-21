pub mod math;
use std::{fs::File, io::Write};

fn render() -> std::io::Result<()> {
    let width: u16 = 256;
    let height: u16 = 256;

    let path = std::path::Path::new("out/scene.ppm");
    let prefix = path.parent().unwrap();
    std::fs::create_dir_all(prefix)?;

    let mut ppm = File::create(path)?;
    write!(ppm, "P3\n{} {}\n255\n", width, height)?;
    for y in 0..height {
        for x in 0..width {
            let r = x as f32 / (width - 1) as f32;
            let g = y as f32 / (height - 1) as f32;
            let b = 0 as f32;

            let ir = (255.999 * r) as u8;
            let ig = (255.999 * g) as u8;
            let ib = (255.999 * b) as u8;

            write!(ppm, "{} {} {}\n", ir, ig, ib)?;
        }
    }
    println!("PPM File Generated.");
    Ok(())
}

fn main() {
    render().err();
}
