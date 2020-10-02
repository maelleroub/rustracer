use super::vec3::Vec3;
use super::ray::Ray;
use super::hittable::Hittable;
use super::hittable::HitRecord;

pub struct Sphere {
    center: Vec3,
    radius: f64
}

impl Hittable for Sphere {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = r.origin - self.center;
        let a = r.direction.norm_squared();
        let half_b = Vec3::dot(oc, r.direction);
        let c = oc.norm_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant > 0.0 {
            let root = f64::sqrt(discriminant);
            let mut temp = (-half_b - root) / a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = r.at(rec.t);
                rec.normal = (rec.p - self.center) / self.radius;
                return true;
            }

            temp = (-half_b + root) / a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = r.at(rec.t);
                rec.normal = (rec.p - self.center) / self.radius;
                return true;
            }
        }

        false
    }
}
