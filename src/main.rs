mod image;
mod vec3;
use vec3::Vec3;
mod ray;
use std::path;
mod hittable;
use hittable::Hittable;
use hittable::HittableList;
mod sphere;
use sphere::Sphere;
mod rt;
mod camera;

fn main() {
    //Image
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width = 400;
    let image_height = ((image_width as f64) / aspect_ratio) as usize;
    let mut image = image::Image {
        width: image_width,
        height: image_height,
        pixels: vec!(Vec3(0.0, 0.0, 0.0); image_width * image_height),
    };

    //World
    let mut world = HittableList::new();
    world.add(Box::new(Sphere {
        center: Vec3(0.0, 0.0, -1.0),
        radius: 0.5
    }));
    world.add(Box::new(Sphere {
        center: Vec3(0.0, -100.5, -1.0),
        radius: 100.0
    }));

    //Camera
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
            let pixel_color = r.color(&world) * image::MAX_RGB_VALUE;
            image.pixels[j * image.width + i] = pixel_color;
        }
    }
    let output_file: &str = "output.ppm";
    image.print(path::Path::new(output_file));
    println!("Wrote output image to {}", output_file);
}
