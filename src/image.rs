use std::fs;
use std::io::Write;
use std::path;
use super::vec3::Vec3;
use super::rt::clamp;

pub struct Image {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<Vec3>,
}

impl Image {
    pub fn adjust_color(pixel_color: Vec3, samples_per_pixel: u32) -> Vec3 {
        let mut r = pixel_color.0;
        let mut g = pixel_color.1;
        let mut b = pixel_color.2;

        let scale = 1.0 / (samples_per_pixel as f64);
        r = f64::sqrt(scale * r);
        g = f64::sqrt(scale * g);
        b = f64::sqrt(scale * b);

        Vec3(
            256.0 * clamp(r, 0.0, 0.999),
            256.0 * clamp(g, 0.0, 0.999),
            256.0 * clamp(b, 0.0, 0.999)
        )
    }

    pub fn print(&self, path: &path::Path) {
        let file = match fs::File::create(&path) {
            Ok(ret) => ret,
            Err(_) => panic!("Could not create output file")
        };
        write_line_to_file(&file, "P3".to_string());
        write_line_to_file(&file, format!("{} {}", self.width, self.height));
        write_line_to_file(&file, 255.to_string());

        for j in (0..self.height).rev() {
            for i in 0..self.width {
                let p = self.pixels[j * self.width + i];
                write_to_file(&file, format!("{} {} {}",
                            p.0 as u64, p.1 as u64, p.2 as u64).to_string());
                let mut c = ' ';
                if i == (self.width - 1) {
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
