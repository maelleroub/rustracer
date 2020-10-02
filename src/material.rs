use super::vec3::Vec3;
use super::ray::Ray;
use super::hittable::HitRecord;

pub trait Material {
    fn scatter(&self, r_in: Ray, rec: HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool;
    fn box_clone(&self) -> Box<Material>;
}

impl Clone for Box<Material> {
    fn clone(&self) -> Box<Material> {
        self.box_clone()
    }
}

#[derive(Clone)]
pub struct Lambertian {
    pub albedo: Vec3
}

impl Lambertian {
    pub fn new() -> Lambertian {
        Lambertian { albedo: Vec3::new() }
    }
}

impl Material for Lambertian {
    fn scatter(&self, r_in: Ray, rec: HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        let scatter_direction = rec.normal + Vec3::random_unit_vector();
        *scattered = Ray {
            origin: rec.p,
            direction: scatter_direction
        };
        *attenuation = self.albedo;
        return true;
    }
    fn box_clone(&self) -> Box<Material> {
        Box::new((*self).clone())
    }
}

#[derive(Clone)]
pub struct Metal {
    pub albedo: Vec3
}

impl Metal {
    pub fn new() -> Metal {
        Metal { albedo: Vec3::new() }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: Ray, rec: HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        let reflected = Vec3::reflect(r_in.direction.normalize(), rec.normal);
        *scattered = Ray {
            origin: rec.p,
            direction: reflected
        };
        *attenuation = self.albedo;
        return Vec3::dot(scattered.direction, rec.normal) > 0.0;
    }
    fn box_clone(&self) -> Box<Material> {
        Box::new((*self).clone())
    }
}
