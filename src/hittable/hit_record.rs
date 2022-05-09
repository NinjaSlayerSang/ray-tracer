use std::fmt::{Display, Formatter, Result};

use crate::point3::Point3;
use crate::vec3::Vec3;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct HitRecord {
    pub p: Point3,
    pub n: Vec3,
    pub t: f64,
}

impl Display for HitRecord {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "N{} at {} with t = {}", self.n, self.p, self.t)
    }
}
