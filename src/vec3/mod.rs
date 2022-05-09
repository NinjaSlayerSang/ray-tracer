#![allow(unused)]

mod ops;
mod utils;

use std::cmp::Ordering;
use std::fmt::{Display, Formatter, Result};

pub use utils::F64Tuple3;

/*
Vec3 is regarded as value type, which implements Copy trait.
Each time when Vec3 called by method or invloved in calculation,
it will be copied as base value type like "integer" or "float".
*/
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl PartialOrd for Vec3 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.length().partial_cmp(&other.length())
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
}
