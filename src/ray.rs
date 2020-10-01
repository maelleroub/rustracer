use super::vec3::Vec3;

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3
}

impl Ray {
    pub fn at(&self, t: f64) -> Vec3 {
        &self.origin + &(&self.direction * t)
    }
    pub fn color(&self) -> Vec3 {
        let unit_direction: Vec3 = self.direction.normalize();
        let t: f64 = 0.5 * (unit_direction.1 + 1.0);
        &(&Vec3(1.0, 1.0, 1.0) * (1.0 - t)) + &(&Vec3(0.5, 0.7, 1.0) * t)
    }
}
