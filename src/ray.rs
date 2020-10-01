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
        let mut t = self.hit_sphere(Vec3(0.0, 0.0, -1.0), 0.5);
        if t > 0.0 {
            let n = (self.at(t) - Vec3(0.0, 0.0, -1.0)).normalize();
            return Vec3(n.0 + 1.0, n.1 + 1.0, n.2 + 1.0) * 0.5;
        }
        let unit_direction = self.direction.normalize();
        t = 0.5 * (unit_direction.1 + 1.0);
        Vec3(1.0, 1.0, 1.0) * (1.0 - t) + (Vec3(0.5, 0.7, 1.0) * t)
    }

    pub fn hit_sphere(&self, center: Vec3, radius: f64) -> f64 {
        let oc = self.origin - center;
        let a = Vec3::dot(self.direction, self.direction);
        let b = Vec3::dot(oc, self.direction) * 2.0;
        let c = Vec3::dot(oc, oc) - radius * radius;
        let discriminant = b * b - 4.0 * a * c;
        if discriminant < 0.0 {
            return -1.0;
        }
        else {
            return (-b - f64::sqrt(discriminant)) / (2.0 * a);
        }
    }
}
