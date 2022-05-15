use crate::{
    color::{primary_color::WHITE, Color},
    hittable::HitRecord,
    ray::Ray,
};

use super::Material;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LightSource {
    light: Color,
}

impl LightSource {
    pub fn new(light: Color) -> Self {
        Self { light }
    }
}

impl Default for LightSource {
    fn default() -> Self {
        Self::new(WHITE)
    }
}

impl Material for LightSource {
    fn scatter(&self, _: &Ray, _: &HitRecord, attenuation: &mut Color, _: &mut Ray) -> bool {
        *attenuation = self.light;
        false
    }
}
