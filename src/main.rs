use std::path;
mod image;
mod vec3;

fn main() {
    let mut image = image::Image {
        width: 512,
        height: 512,
        pixels: Vec::with_capacity(512 * 512),
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
    let a = vec3::Vec3(35.0, 47.5, 12.3);
    let b = vec3::Vec3(35.0, 47.5, 12.3);
    let c = a + b;
    c.print();
}
