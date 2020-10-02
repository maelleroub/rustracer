mod image;
mod vec3;
use vec3::Vec3;
mod ray;
use std::path;
mod hittable;
mod sphere;
mod rt;

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

    let origin = Vec3(0.0, 0.0, 0.0);
    let horizontal = Vec3(viewport_width, 0.0, 0.0);
    let vertical = Vec3(0.0, viewport_height, 0.0);
    let lower_left_corner = origin - (horizontal / 2.0)
        - (vertical / 2.0) - Vec3(0.0, 0.0, focal_length);

    for j in (0..image.height).rev() {
        for i in 0..image.width {
            let u: f64 = (i as f64) / ((image_width - 1) as f64);
            let v: f64 = (j as f64) / ((image_height - 1) as f64);
            let r = ray::Ray {
                origin: origin,
                direction: lower_left_corner + (horizontal * u)
                    + (vertical * v) - origin,
            };
            image.pixels[j * image.width + i] = r.color() * image::MAX_RGB_VALUE;
        }
    }
    let output_file: &str = "output.ppm";
    image.print(path::Path::new(output_file));
    println!("Wrote output image to {}", output_file);
}
