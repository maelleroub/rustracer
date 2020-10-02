use rand::Rng;

pub const INFINITY : f64 = core::f64::MAX;
pub const PI : f64 = 3.1415926535897932385;

#[inline]
pub fn degress_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

#[inline]
pub fn random_double() -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen::<f64>()
}

#[inline]
pub fn random_double_range(min: f64, max: f64) -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(min, max)
}
