use std::fs;
use std::io::Write;
use std::path;

pub const MAX_RGB_VALUE: u8 = 255;

pub struct Pixel(pub u8, pub u8, pub u8);

pub struct Image {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<Pixel>,
}

impl Image {
    pub fn print(&self, path: &path::Path) {
        let file = fs::File::create(&path).expect("Could not create output file");
        write_line_to_file(&file, "P3".to_string());
        write_line_to_file(&file, format!("{} {}", self.width, self.height));
        write_line_to_file(&file, MAX_RGB_VALUE.to_string());

        for i in 0..self.height {
            for j in 0..self.width {
                let p = &self.pixels[i * self.width + j];
                write_to_file(&file, format!("{} {} {}", p.0, p.1, p.2).to_string());
                let mut c = ' ';
                if j == (self.width - 1) {
                    c = '\n';
                }
                write_to_file(&file, c.to_string());
            }
        }
    }
}

fn write_line_to_file(file: &fs::File, line: String) {
    write_to_file(file, line + "\n");
}

fn write_to_file(mut file: &fs::File, line: String) {
    file.write_all((line).as_bytes()).expect("Could not write to file");
}
