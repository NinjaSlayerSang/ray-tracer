use std::{
    fmt::{Display, Formatter, Result},
    rc::Rc,
};

use crate::{
    material::{Empty, Material},
    vec3::Vec3,
};

#[derive(Clone)]
pub struct HitRecord {
    pub t: f64,
    pub normal: Vec3,
    pub material: Rc<dyn Material>,
}

impl Default for HitRecord {
    fn default() -> Self {
        Self {
            t: f64::default(),
            normal: Vec3::default(),
            material: Rc::new(Empty::default()),
        }
    }
}

impl Display for HitRecord {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "N{} at t = {}", self.normal, self.t)
    }
}
