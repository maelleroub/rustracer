use std::path;
mod image;

fn main() {
    let mut image = image::Image {
        width: 1024,
        height: 1024,
        pixels: Vec::with_capacity(1024 * 1024),
    };
    for i in 0..image.height {
        for j in 0..image.width {
            let ratio = image::MAX_RGB_VALUE as f64;
            let red = ((i as f64 / image.height as f64) * ratio) as u8;
            let green = 255 - ((i as f64 / image.height as f64) * ratio) as u8;
            let blue = ((j as f64 / image.width as f64) * ratio) as u8;
            image.pixels.push(image::Pixel(red, green, blue));
        }
    }
    image.print(path::Path::new("output.ppm"));
}
