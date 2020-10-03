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

fn random_scene() -> HittableList {
    let mut world = HittableList::new();
    let material_ground = Box::new(Lambertian::new_albedo(Vec3(0.5, 0.5, 0.5)));
    world.add(Box::new(Sphere::new_center_radius_mat(
                        Vec3(0.0, -1000.0, 0.0),
                        1000.0,
                        material_ground)));
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rt::random_double();
            let center = Vec3((a as f64) + 0.9 * rt::random_double(),
            0.2,
            (b as f64) + 0.9 * rt::random_double());

            if (center - Vec3(4.0, 0.2, 0.0)).norm() > 0.9 {
                if choose_mat < 0.8 {
                    //Diffuse
                    let albedo = Vec3::random() * Vec3::random();
                    let mat = Box::new(Lambertian::new_albedo(albedo));
                    world.add(Box::new(Sphere::new_center_radius_mat(center, 0.2, mat)));
                } else if choose_mat < 0.95 {
                    //Metal
                    let albedo = Vec3::random_range(0.5, 1.0);
                    let fuzz = rt::random_double_range(0.0, 0.5);
                    let mat = Box::new(Metal::new_albedo_fuzz(albedo, fuzz));
                    world.add(Box::new(Sphere::new_center_radius_mat(center, 0.2, mat)));
                } else {
                    //Glass
                    let mat = Box::new(Dielectric::new_ref_idx(1.5));
                    world.add(Box::new(Sphere::new_center_radius_mat(center, 0.2, mat)));
                }
            }
        }
    }
    let material1 = Box::new(Dielectric::new_ref_idx(1.5));
    world.add(Box::new(Sphere::new_center_radius_mat(Vec3(0.0, 1.0, 0.0), 1.0, material1)));

    let material2 = Box::new(Lambertian::new_albedo(Vec3(0.4, 0.2, 0.1)));
    world.add(Box::new(Sphere::new_center_radius_mat(Vec3(-4.0, 1.0, 0.0), 1.0, material2)));

    let material3 = Box::new(Metal::new_albedo_fuzz(Vec3(0.7, 0.6, 0.5), 0.0));
    world.add(Box::new(Sphere::new_center_radius_mat(Vec3(4.0, 1.0, 0.0), 1.0, material3)));

    return world;
}

fn main() {
    //Image
    let aspect_ratio: f64 = 3.0 / 2.0;
    let image_width = 800;
    let image_height = ((image_width as f64) / aspect_ratio) as usize;
    let samples_per_pixel = 50;
    let max_depth = 50;

    let mut image = image::Image {
        width: image_width,
        height: image_height,
        pixels: vec!(Vec3(0.0, 0.0, 0.0); image_width * image_height),
    };

    //World
    let world = random_scene();

    //Camera
    let lookfrom = Vec3(13.0, 2.0, 3.0);
    let lookat = Vec3(0.0, 0.0, 0.0);
    let vup = Vec3(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;
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
        println!("Finished line {}", j);
    }
    let output_file: &str = "output.ppm";
    image.print(path::Path::new(output_file));
    println!("Wrote output image to {}", output_file);
}
