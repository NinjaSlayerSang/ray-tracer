#![allow(unused)]

use crate::point3::Point3;

#[derive(Clone, Debug, Default)]
pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Self {
        Self { center, radius }
    }
}
