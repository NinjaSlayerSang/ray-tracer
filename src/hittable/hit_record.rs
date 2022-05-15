use std::rc::Rc;

use crate::{
    material::{Context, Empty, Material},
    vec3::Vec3,
};

#[derive(Clone)]
pub struct HitRecord {
    pub t: f64,
    pub normal: Vec3,
    pub material: Rc<dyn Material>,
    pub ctx: Context,
}

impl Default for HitRecord {
    fn default() -> Self {
        Self {
            t: f64::default(),
            normal: Vec3::default(),
            material: Rc::new(Empty::default()),
            ctx: Context::default(),
        }
    }
}
