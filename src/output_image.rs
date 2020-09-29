use std::fs;
use std::io::Write;
use std::path;

pub struct Pixel(pub u8, pub u8, pub u8);

pub struct Image {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<Pixel>,
}

pub fn print_image(image: &Image, path: &path::Path) {
    let mut file = fs::File::create(&path).expect("Could not create output file");
    file.write_all("P3\n".as_bytes()).expect("Could not write to file");
    file.write_all((format!("{} {}\n", image.width, image.height)).as_bytes()).expect("Could not write to file");
    file.write_all("255\n".as_bytes()).expect("Could not write to file");

    for i in 0..image.height {
        for j in 0..image.width {
            let p = &image.pixels[i * image.width + j];
            file.write_all((format!("{} {} {}", p.0, p.1, p.2)).as_bytes()).expect("Could not write to file");
            let mut c = ' ';
            if j == (image.width - 1) {
                c = '\n';
            }
            file.write_all(c.to_string().as_bytes()).expect("Could not write to file");
        }
    }
}
