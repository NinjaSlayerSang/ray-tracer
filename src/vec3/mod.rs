mod ops;

use std::{
    cmp::Ordering,
    fmt::{Display, Formatter, Result},
};

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

type F64Tuple3 = (f64, f64, f64);

impl From<F64Tuple3> for Vec3 {
    fn from(tuple: F64Tuple3) -> Self {
        Self {
            x: tuple.0,
            y: tuple.1,
            z: tuple.2,
        }
    }
}

impl Into<F64Tuple3> for Vec3 {
    fn into(self) -> F64Tuple3 {
        (self.x, self.y, self.z)
    }
}
