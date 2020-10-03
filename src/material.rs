use super::vec3::Vec3;
use super::ray::Ray;
use super::hittable::HitRecord;

pub trait Material {
    fn scatter(&self, r_in: Ray, rec: HitRecord, attenuation: &mut Vec3,
    scattered: &mut Ray) -> bool;
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
    fn scatter(&self, r_in: Ray, rec: HitRecord, attenuation: &mut Vec3,
    scattered: &mut Ray) -> bool {
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
    pub albedo: Vec3,
    pub fuzz: f64
}

impl Metal {
    pub fn new() -> Metal {
        Metal::new_albedo_fuzz(Vec3(0.0, 0.0, 0.0), 0.0)
    }

    pub fn new_albedo_fuzz(a: Vec3, f: f64) -> Metal {
        Metal {
            albedo: a,
            fuzz: if f < 1.0 { f } else { 1.0 }
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: Ray, rec: HitRecord, attenuation: &mut Vec3,
    scattered: &mut Ray) -> bool {
        let reflected = Vec3::reflect(r_in.direction.normalize(), rec.normal);
        *scattered = Ray {
            origin: rec.p,
            direction: reflected + self.fuzz * Vec3::random_in_unit_sphere()
        };
        *attenuation = self.albedo;
        return Vec3::dot(scattered.direction, rec.normal) > 0.0;
    }

    fn box_clone(&self) -> Box<Material> {
        Box::new((*self).clone())
    }
}

#[derive(Clone)]
pub struct Dielectric {
    pub ref_idx: f64
}

impl Dielectric {
    pub fn new() -> Dielectric {
        Dielectric { ref_idx: 0.0 }
    }
    pub fn new_ref_idx(f: f64) -> Dielectric {
        Dielectric { ref_idx: f }
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: Ray, rec: HitRecord, attenuation: &mut Vec3,
    scattered: &mut Ray) -> bool {
        *attenuation = Vec3(1.0, 1.0, 1.0);
        let etai_over_etat = if rec.front_face {
            1.0 / self.ref_idx
        } else {
            self.ref_idx
        };
        let unit_direction = r_in.direction.normalize();

        let cos_theta = f64::min(Vec3::dot((-1.0) * unit_direction, rec.normal),
                                1.0);
        let sin_theta = f64::sqrt(1.0 - cos_theta * cos_theta);
        if etai_over_etat * sin_theta > 1.0 {
            let reflected = Vec3::reflect(unit_direction, rec.normal);
            *scattered = Ray {
                origin: rec.p,
                direction: reflected
            };
        }

        let refracted = Vec3::refract(unit_direction, rec.normal, etai_over_etat);
        *scattered = Ray {
            origin: rec.p,
            direction: refracted
        };
        return true;
    }

    fn box_clone(&self) -> Box<Material> {
        Box::new((*self).clone())
    }
}
