mod ops;
mod utils;

#[derive(Copy, Clone, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub fn zero() -> Vec3 {
    Vec3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    }
}

pub fn unit() -> Vec3 {
    Vec3 {
        x: 1.0,
        y: 1.0,
        z: 1.0,
    }
}
