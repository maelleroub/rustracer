use std::fs;
use std::io::Write;
use std::path;

pub struct Pixel(pub u8, pub u8, pub u8);

pub struct Image {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<Pixel>,
}

fn write_line_to_file(file: &fs::File, line: String) {
    write_to_file(file, line + "\n");
}

fn write_to_file(mut file: &fs::File, line: String) {
    file.write_all((line).as_bytes()).expect("Could not write to file");
}

pub fn print_image(image: &Image, path: &path::Path) {
    let file = fs::File::create(&path).expect("Could not create output file");
    write_line_to_file(&file, "P3".to_string());
    write_line_to_file(&file, format!("{} {}", image.width, image.height));
    write_line_to_file(&file, "255".to_string());

    for i in 0..image.height {
        for j in 0..image.width {
            let p = &image.pixels[i * image.width + j];
            write_to_file(&file, format!("{} {} {}", p.0, p.1, p.2).to_string());
            let mut c = ' ';
            if j == (image.width - 1) {
                c = '\n';
            }
            write_to_file(&file, c.to_string());
        }
    }
}
