use std::{
    fs::{self, File},
    io::{BufWriter, Write},
};

pub struct PPMImage {
    buffer: Vec<u8>,
}

impl PPMImage {
    pub fn new(width: u32, height: u32) -> Self {
        let mut image = Self { buffer: Vec::new() };
        image.buffer.extend("P6\n".as_bytes());
        image
            .buffer
            .extend(format!("{} {}\n", width, height).as_bytes());
        image.buffer.extend("255\n".as_bytes());
        image
    }

    pub fn write_color(&mut self, r: u8, g: u8, b: u8) -> Result<(), std::io::Error> {
        self.buffer.extend(r.to_be_bytes());
        self.buffer.extend(g.to_be_bytes());
        self.buffer.extend(b.to_be_bytes());
        Ok(())
    }

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
