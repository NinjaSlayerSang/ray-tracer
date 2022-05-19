mod ops;
mod utils;

use std::cmp::Ordering;

/*
Vec3 is regarded as value type, which implements Copy trait.
Each time when Vec3 called by method or invloved in calculation,
it will be copied as base value type like "integer" or "float".
*/
#[derive(Clone, Copy, Default, PartialEq)]
pub struct Vec3(f64, f64, f64);

impl PartialOrd for Vec3 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.length().partial_cmp(&other.length())
    }
}

impl From<(f64, f64, f64)> for Vec3 {
    fn from(f: (f64, f64, f64)) -> Self {
        Self(f.0, f.1, f.2)
    }
}

impl Vec3 {
    pub fn new(x: impl Into<f64>, y: impl Into<f64>, z: impl Into<f64>) -> Self {
        Self(x.into(), y.into(), z.into())
    }

    // alias

    pub fn x(&self) -> f64 {
        self.0
    }

    pub fn y(&self) -> f64 {
        self.1
    }

    pub fn z(&self) -> f64 {
        self.2
    }
}
