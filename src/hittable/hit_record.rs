use std::{
    fmt::{Display, Formatter, Result},
    rc::Rc,
};

use crate::{
    material::{Empty, Material},
    point3::Point3,
    ray::Ray,
    vec3::Vec3,
};

#[derive(Clone)]
pub struct HitRecord {
    pub t: f64,
    pub point: Point3,
    pub normal: Vec3,
    pub front_face: bool,
    pub material: Rc<dyn Material>,
}

impl Default for HitRecord {
    fn default() -> Self {
        Self {
            t: f64::default(),
            point: Point3::default(),
            normal: Vec3::default(),
            front_face: bool::default(),
            material: Rc::new(Empty::default()),
        }
    }
}

impl Display for HitRecord {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "N{} at {} with t = {}", self.normal, self.point, self.t)
    }
}

impl HitRecord {
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        self.front_face = Vec3::dot(ray.direction, outward_normal).is_sign_negative();
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}
