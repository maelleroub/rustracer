pub const INFINITY : f64 = core::f64::MAX;
pub const PI : f64 = 3.1415926535897932385;

#[inline]
pub fn degress_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}
