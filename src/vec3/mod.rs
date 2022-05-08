#![allow(unused)]

mod ops;
mod utils;

pub use utils::F64Tuple3;

/*
Vec3 is regarded as value type, which implements Copy trait.
Each time when Vec3 called by method or invloved in calculation,
it will be copy as base value type like "number".
*/
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
}

pub fn zero() -> Vec3 {
    Vec3::default()
}
