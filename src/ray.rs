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

    pub fn color(&self, world: &impl Hittable, depth: i32) -> Vec3 {
        let mut rec = HitRecord::new();

        if depth <= 0 {
            return Vec3(0.0, 0.0, 0.0);
        }

        if world.hit(*self, 0.001, rt::INFINITY, &mut rec) {
            let target = rec.p + rec.normal + Vec3::random_unit_vector();
            let r = Ray {
                origin: rec.p,
                direction: target - rec.p
            };
            return 0.5 * r.color(world, depth - 1);
        }
        let unit_direction = self.direction.normalize();
        let t = 0.5 * (unit_direction.1 + 1.0);
        return (1.0 - t) * Vec3(1.0, 1.0, 1.0) + t * Vec3(0.5, 0.7, 1.0);
    }
}
