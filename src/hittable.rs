use super::ray::Ray;
use super::vec3::Vec3;

pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f64
}

trait Hittable {
    fn hit(r: Ray, t_min: f64, t_max: f64, rec: HitRecord) -> bool;
}
