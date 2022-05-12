use std::fmt::{Display, Formatter, Result};

use crate::{point3::Point3, ray::Ray, vec3::Vec3};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct HitRecord {
    pub t: f64,
    pub point: Point3,
    pub normal: Vec3,
    pub front_face: bool,
}

impl Display for HitRecord {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "N{} at {} with t = {}", self.normal, self.point, self.t)
    }
}

impl HitRecord {
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        self.front_face = ray.direction * outward_normal < 0f64;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}
