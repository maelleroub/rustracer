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
    pub fn new() -> Ray {
        Ray {
            origin: Vec3::new(),
            direction: Vec3::new()
        }
    }

    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + (self.direction * t)
    }

    pub fn color(&self, world: &impl Hittable, depth: i32) -> Vec3 {
        let mut rec = HitRecord::new();

        if depth <= 0 {
            return Vec3(0.0, 0.0, 0.0);
        }

        if world.hit(*self, 0.001, rt::INFINITY, &mut rec) {
            let mut scattered = Ray::new();
            let mut attenuation = Vec3::new();
            if rec.mat_ptr.scatter(*self, rec.clone(), &mut attenuation, &mut scattered) {
                return attenuation * scattered.color(world, depth - 1);
            }
            return Vec3::new();
        }
        let unit_direction = self.direction.normalize();
        let t = 0.5 * (unit_direction.1 + 1.0);
        return (1.0 - t) * Vec3(1.0, 1.0, 1.0) + t * Vec3(0.5, 0.7, 1.0);
    }
}
