use super::rt;
use super::vec3::Vec3;
use super::hittable::HitRecord;
use super::hittable::Hittable;

#[derive(Clone, Copy)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3
}

impl Ray {
    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + (self.direction * t)
    }

    pub fn color(&self, world: &impl Hittable) -> Vec3 {
        let mut rec = HitRecord::new();
        if world.hit(*self, 0.0, rt::INFINITY, &mut rec) {
            return 0.5 * (rec.normal + Vec3(1.0, 1.0, 1.0));
        }
        let unit_direction = self.direction.normalize();
        let t = 0.5 * (unit_direction.1 + 1.0);
        return (1.0 - t) * Vec3(1.0, 1.0, 1.0) + t * Vec3(0.5, 0.7, 1.0);
    }

    pub fn hit_sphere(&self, center: Vec3, radius: f64) -> f64 {
        let oc = self.origin - center;
        let a = self.direction.norm_squared();
        let half_b = Vec3::dot(oc, self.direction);
        let c = oc.norm_squared() - radius * radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return -1.0;
        }
        else {
            return (-half_b - f64::sqrt(discriminant)) / a;
        }
    }
}
