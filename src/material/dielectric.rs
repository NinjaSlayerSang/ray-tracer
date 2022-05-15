use crate::{
    color::{primary_color::WHITE, Color},
    hittable::HitRecord,
    ray::Ray,
    utils::refleract,
};

use super::Material;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Dielectric {
    ir: f64,
}

impl Dielectric {
    pub fn new(ir: f64) -> Self {
        Self { ir }
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        ray_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        *attenuation = WHITE;
        *scattered = Ray::new(
            ray_in.at(rec.t),
            refleract(ray_in.direction, rec.normal, self.ir, 0f64),
        );
        true
    }
}
