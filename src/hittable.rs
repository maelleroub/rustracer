use super::ray::Ray;
use super::vec3::Vec3;

#[derive(Clone, Copy)]
pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool
}

impl HitRecord {
    pub fn new() -> HitRecord {
        return HitRecord {
            p: Vec3::new(),
            normal: Vec3::new(),
            t: 0.0,
            front_face: false,
        };
    }
    #[inline]
    pub fn set_face_normal(&mut self, r: Ray, outward_normal: Vec3) {
        self.front_face = Vec3::dot(r.direction, outward_normal) < 0.0;
        self.normal = outward_normal * if self.front_face {1.0} else {-1.0};
    }
}

pub trait Hittable {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}

pub struct HittableList {
    pub objects: Vec<Box<Hittable>>
}

impl HittableList {
    pub fn new() -> HittableList {
        return HittableList{
            objects: Vec::new(),
        };
    }
    pub fn add(&mut self, object: Box<Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::new();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for object in &self.objects {
            if object.hit(r, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec;
            }
        }
        return hit_anything;
    }
}
