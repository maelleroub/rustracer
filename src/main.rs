mod image;
mod vec3;
use vec3::Vec3;
mod ray;
use std::path;
mod hittable;

fn main() {
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width = 400;
    let image_height = ((image_width as f64) / aspect_ratio) as usize;
    let mut image = image::Image {
        width: image_width,
        height: image_height,
        pixels: vec!(Vec3(0.0, 0.0, 0.0); image_width * image_height),
    };

    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin1 = Vec3(0.0, 0.0, 0.0);
    let horizontal = Vec3(viewport_width, 0.0, 0.0);
    let vertical = Vec3(0.0, viewport_height, 0.0);
    let lower_left_corner = origin1 - (horizontal / 2.0)
        - (vertical / 2.0) - Vec3(0.0, 0.0, focal_length);

    for j in (0..image.height).rev() {
        for i in 0..image.width {
            let u: f64 = (i as f64) / ((image_width - 1) as f64);
            let v: f64 = (j as f64) / ((image_height - 1) as f64);
            let origin = Vec3(0.0, 0.0, 0.0);
            let r = ray::Ray {
                direction: lower_left_corner + (horizontal * u)
                    + (vertical * v) - origin,
                origin: origin,
            };
            image.pixels[j * image.width + i] = r.color() * image::MAX_RGB_VALUE;
        }
    }
    image.print(path::Path::new("output.ppm"));
    let mut a = vec3::Vec3(10.0, 5.0, 0.0);
    println!("a = {}", a.to_string());
    let b = vec3::Vec3(102.8, 47.5, 12.3);
    println!("b = {}", b.to_string());
    a += b;
    println!("a + b = {}", a.to_string());
    a *= 15.0;
    println!("a * 15 = {}", a.to_string());
    a = a - (a / 2.0);
    println!("a - (a / 2.0) = {}", a.to_string());
    let c = a + b;
    println!("a + b = {}", c.to_string());
    println!("Normalized: {}", c.normalize().to_string());
    let r = ray::Ray {
        origin: a,
        direction: b,
    };
    println!("Ray at 55: {}", r.at(55.0));
}
