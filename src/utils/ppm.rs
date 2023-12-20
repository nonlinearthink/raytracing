use std::{
    fs::{self, File},
    io::{BufWriter, Write},
};

/**
A tool to write PPM image.

PPM(Portable Pixmap Format) is a simple and easy to understand image format. For more detail: [Netpbm](https://en.wikipedia.org/wiki/Netpbm).
*/
pub struct PPMImage {
    buffer: Vec<u8>,
}

impl PPMImage {
    /**
    Create a new `PPMImage` with image height and width.

    It will also init an image buffer defined by **Binary Portable PixMap**, which starts with a header called `P6` in PPM.
     */
    pub fn new(width: u32, height: u32) -> Self {
        let mut image = Self { buffer: Vec::new() };
        image.buffer.extend("P6\n".as_bytes());
        image
            .buffer
            .extend(format!("{} {}\n", width, height).as_bytes());
        image.buffer.extend("255\n".as_bytes());
        image
    }

    /// Write a RGB color to the image buffer.
    pub fn write_color(&mut self, r: u8, g: u8, b: u8) -> Result<(), std::io::Error> {
        self.buffer.extend(r.to_be_bytes());
        self.buffer.extend(g.to_be_bytes());
        self.buffer.extend(b.to_be_bytes());
        Ok(())
    }

    /// Save the image buffer as a file named `save_path`.
    pub fn save(&mut self, save_path: String) -> Result<(), std::io::Error> {
        let path = std::path::Path::new(&save_path);
        let prefix = path.parent().unwrap();
        fs::create_dir_all(prefix)?;

        let file = File::create(path)?;
        let mut writer = BufWriter::new(file);
        writer.write_all(&self.buffer)?;

        Ok(())
    }
}
