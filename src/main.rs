use rst_raytrace::core::Vector3 as Color3;
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
            let color = Color3::new(
                f32::from(x) / f32::from(width - 1),
                f32::from(y) / f32::from(height - 1),
                0.,
            );

            let ir = (255.999 * color.x) as u8;
            let ig = (255.999 * color.y) as u8;
            let ib = (255.999 * color.z) as u8;

            write!(ppm, "{} {} {}\n", ir, ig, ib)?;
        }
    }
    println!("PPM File Generated.");
    Ok(())
}

fn main() {
    render().err();
}
