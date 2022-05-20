use std::sync::Arc;

use crate::{
    material::{Context, Empty, Material},
    vec3::Vec3,
};

#[derive(Clone)]
pub struct HitRecord {
    pub t: f64,
    pub normal: Vec3,
    pub material: Arc<dyn Material>,
    pub ctx: Context,
}

impl Default for HitRecord {
    fn default() -> Self {
        Self {
            t: Default::default(),
            normal: Default::default(),
            material: Arc::new(Empty::default()),
            ctx: Default::default(),
        }
    }
}
