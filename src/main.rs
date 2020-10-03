mod image;
use image::Image;
mod vec3;
use vec3::Vec3;
mod ray;
use std::path;
mod hittable;
use hittable::HittableList;
mod sphere;
use sphere::Sphere;
mod rt;
mod camera;
use camera::Camera;
mod material;
use material::Lambertian;
use material::Metal;
use material::Dielectric;

fn main() {
    //Image
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width = 400;
    let image_height = ((image_width as f64) / aspect_ratio) as usize;
    let samples_per_pixel = 50;
    let max_depth = 50;

    let mut image = image::Image {
        width: image_width,
        height: image_height,
        pixels: vec!(Vec3(0.0, 0.0, 0.0); image_width * image_height),
    };

    //World
    let R = f64::cos(rt::PI / 4.0);
    let mut world = HittableList::new();

    let material_ground = Box::new(Lambertian::new_albedo(Vec3(0.8, 0.8, 0.0)));
    let material_center = Box::new(Lambertian::new_albedo(Vec3(0.1, 0.2, 0.5)));
    let material_left = Box::new(Dielectric::new_ref_idx(1.5));
    let material_right = Box::new(Metal::new_albedo_fuzz(Vec3(0.8, 0.6, 0.2), 0.0));

    world.add(Box::new(Sphere::new_center_radius_mat(
                        Vec3(0.0, -100.5, -1.0),
                        100.0,
                        material_ground)));
    world.add(Box::new(Sphere::new_center_radius_mat(
                        Vec3(0.0, 0.0, -1.0),
                        0.5,
                        material_center)));
    world.add(Box::new(Sphere::new_center_radius_mat(
                        Vec3(-1.0, 0.0, -1.0),
                        0.5,
                        material_left.clone())));
    world.add(Box::new(Sphere::new_center_radius_mat(
                        Vec3(-1.0, 0.0, -1.0),
                        -0.45,
                        material_left)));
    world.add(Box::new(Sphere::new_center_radius_mat(
                        Vec3(1.0, 0.0, -1.0),
                        0.5,
                        material_right)));

    //Camera
    let lookfrom = Vec3(3.0, 3.0, 2.0);
    let lookat = Vec3(0.0, 0.0, -1.0);
    let vup = Vec3(0.0, 1.0, 0.0);
    let dist_to_focus = (lookfrom - lookat).norm();
    let aperture = 2.0;
    let camera = Camera::new(lookfrom, lookat, vup, 20.0, aspect_ratio,
                             aperture, dist_to_focus);

    for j in (0..image.height).rev() {
        for i in 0..image.width {
            let mut pixel_color = Vec3(0.0, 0.0, 0.0);
            for _s in 0..samples_per_pixel {
                let u = ((i as f64) + rt::random_double())
                        / ((image.width - 1) as f64);
                let v = ((j as f64) + rt::random_double())
                        / ((image.height - 1) as f64);
                let r = camera.get_ray(u, v);
                pixel_color += r.color(&world, max_depth);
            }
            pixel_color = Image::adjust_color(pixel_color, samples_per_pixel);
            image.pixels[j * image.width + i] = pixel_color;
        }
    }
    let output_file: &str = "output.ppm";
    image.print(path::Path::new(output_file));
    println!("Wrote output image to {}", output_file);
}
