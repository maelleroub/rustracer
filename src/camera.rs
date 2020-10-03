use super::vec3::Vec3;
use super::ray::Ray;
use super::rt;

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f64
}

impl Camera {
    pub fn new(lookfrom: Vec3, lookat: Vec3, vup: Vec3, vfov: f64,
    aspect_ratio: f64, aperture: f64, focus_dist: f64) -> Camera {
        let theta = rt::degrees_to_radians(vfov);
        let h = f64::tan(theta / 2.0);
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let focal_length = 1.0;

        let w = (lookfrom - lookat).normalize();
        let u = Vec3::cross(vup, w).normalize();
        let v = Vec3::cross(w, u);

        let origin = lookfrom;
        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0
                                - focus_dist * w;
        let lens_radius = aperture / 2.0;

        return Camera {
            origin: origin,
            horizontal: horizontal,
            vertical: vertical,
            lower_left_corner: lower_left_corner,
            u : u,
            v: v,
            w: w,
            lens_radius: lens_radius
        };
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = self.lens_radius * Vec3::random_in_unit_disk();
        let offset = self.u * rd.0 + self.v * rd.1;

        return Ray::new_origin_direction(self.origin + offset,
                    self.lower_left_corner + s * self.horizontal
                    + t * self.vertical - self.origin - offset);
    }
}
