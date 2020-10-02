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
use camera::Camera;

fn main() {
    //Image
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width = 400;
    let image_height = ((image_width as f64) / aspect_ratio) as usize;
    let samples_per_pixel = 100;

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
    let camera = Camera::new();

    for j in (0..image.height).rev() {
        for i in 0..image.width {
            let mut pixel_color = Vec3(0.0, 0.0, 0.0);
            for _s in 0..samples_per_pixel {
                let u = ((i as f64) + rt::random_double())
                        / ((image.width - 1) as f64);
                let v = ((j as f64) + rt::random_double())
                        / ((image.height - 1) as f64);
                let r = camera.get_ray(u, v);
                pixel_color += r.color(&world);
            }
            image.pixels[j * image.width + i] = pixel_color;
        }
    }
    let output_file: &str = "output.ppm";
    image.print(path::Path::new(output_file), samples_per_pixel);
    println!("Wrote output image to {}", output_file);
}
