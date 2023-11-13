use super::Texture;
use crate::core::{Color3, Interval, Point3, Vector2};
use image::{ImageResult, RgbImage};

#[derive(Debug)]
pub struct ImageTexture {
    image: RgbImage,
}

impl ImageTexture {
    pub fn new(path: String) -> ImageResult<Self> {
        let image = image::io::Reader::open(path)?.decode()?.to_rgb8();
        ImageResult::Ok(Self { image })
    }
}

impl Texture for ImageTexture {
    fn value(&self, uv: &Vector2, _point: &Point3) -> Color3 {
        // If we have no texture data, then return solid cyan as a debugging aid.
        if self.image.height() <= 0 {
            return Color3::new(0., 1., 1.);
        }

        // Clamp input texture coordinates to [0,1] x [1,0]
        let u = Interval::new(0., 1.).clamp(uv.u());
        let v = 1.0 - Interval::new(0., 1.).clamp(uv.v()); // Flip V to image coordinates

        let i = u * self.image.width() as f32;
        let j = v * self.image.height() as f32;
        let pixel = self.image.get_pixel(i as u32, j as u32);

        let color_scale = 1.0 / 255.0;
        Color3::new(
            color_scale * pixel[0] as f32,
            color_scale * pixel[1] as f32,
            color_scale * pixel[2] as f32,
        )
    }
}
