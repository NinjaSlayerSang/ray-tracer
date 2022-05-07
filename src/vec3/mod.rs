#![allow(unused)]

mod ops;
mod utils;

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub fn zero() -> Vec3 {
    Vec3::default()
}
