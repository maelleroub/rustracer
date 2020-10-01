use std::path;
mod image;
mod vec3;
mod ray;

fn main() {
    let mut image = image::Image {
        width: 512,
        height: 512,
        pixels: Vec::with_capacity(512 * 512),
    };
    for i in 0..image.height {
        for j in 0..image.width {
            let ratio = image::MAX_RGB_VALUE;
            let red = (i as f64 / image.height as f64) * ratio;
            let green = 255.0 - ((i as f64 / image.height as f64) * ratio);
            let blue = (j as f64 / image.width as f64) * ratio;
            image.pixels.push(vec3::Vec3(red, green, blue));
        }
    }
    image.print(path::Path::new("output.ppm"));
    let mut a = vec3::Vec3(10.0, 5.0, 0.0);
    println!("a = {}", a.to_string());
    let b = vec3::Vec3(102.8, 47.5, 12.3);
    println!("b = {}", b.to_string());
    a += &b;
    println!("a + b = {}", a.to_string());
    a *= 15.0;
    println!("a * 15 = {}", a.to_string());
    a = &a - &(&a / 2.0);
    println!("a - (a / 2.0) = {}", a.to_string());
    let c = &a + &b;
    println!("a + b = {}", c.to_string());
    println!("Normalized: {}", c.normalize().to_string());
    let r = ray::Ray {
        origin: a,
        direction: b,
    };
    println!("Ray at 55: {}", r.at(55.0));
}
