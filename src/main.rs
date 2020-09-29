use std::path;
mod output_image;

fn main() {
    let mut image = output_image::Image {
        width: 3,
        height: 2,
        pixels: Vec::with_capacity(3 * 2),
    };
    image.pixels.push(output_image::Pixel(255, 0, 125));
    image.pixels.push(output_image::Pixel(125, 125, 125));
    image.pixels.push(output_image::Pixel(0, 0, 255));
    image.pixels.push(output_image::Pixel(50, 255, 0));
    image.pixels.push(output_image::Pixel(0, 0, 0));
    image.pixels.push(output_image::Pixel(200, 100, 0));
    output_image::print_image(&image, path::Path::new("output.ppm"));
}
