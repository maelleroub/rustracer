use super::vec3::Vec3;

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3
}

impl Ray {
    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + (self.direction * t)
    }
    pub fn color(&self) -> Vec3 {
        if self.hit_sphere(Vec3(0.0, 0.0, -1.0), 0.5) {
            return Vec3(1.0, 0.0, 0.0);
        }
        let unit_direction: Vec3 = self.direction.normalize();
        let t: f64 = 0.5 * (unit_direction.1 + 1.0);
        Vec3(1.0, 1.0, 1.0) * (1.0 - t) + (Vec3(0.5, 0.7, 1.0) * t)
    }
    pub fn hit_sphere(&self, center: Vec3, radius: f64) -> bool {
        let oc = self.origin - center;
        let a = Vec3::dot(self.direction, self.direction);
        let b = Vec3::dot(oc, self.direction) * 2.0;
        let c = Vec3::dot(oc, oc) - radius * radius;
        let discriminant = b * b - 4.0 * a * c;
        discriminant > 0.0
    }
}
