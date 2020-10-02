use super::vec3::Vec3;
use super::ray::Ray;

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3
}

impl Camera {
    pub fn new() -> Camera {
        let aspect_ratio = 16.0 / 9.0;
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;

        let origin = Vec3(0.0, 0.0, 0.0);
        let horizontal = Vec3(viewport_width, 0.0, 0.0);
        let vertical = Vec3(0.0, viewport_height, 0.0);
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0
                                - Vec3(0.0, 0.0, focal_length);

        return Camera {
            origin: origin,
            horizontal: horizontal,
            vertical: vertical,
            lower_left_corner: lower_left_corner
        };
    }
    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray {
            origin: self.origin,
            direction: self.lower_left_corner + u * self.horizontal
                        + v * self.vertical - self.origin
        }
    }
}
