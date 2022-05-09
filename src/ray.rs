#![allow(unused)]

use std::fmt::{Display, Formatter, Result};

use crate::point3::Point3;
use crate::vec3::Vec3;

#[derive(Clone, Debug, PartialEq)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

impl Display for Ray {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{} + t{}", self.origin, self.direction)
    }
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.origin + t * self.direction
    }
}
