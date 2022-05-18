use crate::{color::Color, hittable::HitRecord, ray::Ray};

use super::Material;

#[derive(Clone, Copy)]
pub struct LightSource {
    light: Color,
}

impl Default for LightSource {
    fn default() -> Self {
        Self {
            light: Color::white(),
        }
    }
}

impl LightSource {
    pub fn new(light: Color) -> Self {
        Self { light }
    }
}

impl Material for LightSource {
    fn scatter(&self, _: &Ray, _: &HitRecord, attenuation: &mut Color, _: &mut Ray) -> bool {
        *attenuation = self.light;
        false
    }
}
