use crate::{color::Color, hittable::HitRecord, ray::Ray, utils::refleract};

use super::Material;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Self {
            albedo,
            fuzz: fuzz.clamp(0f64, 1f64),
        }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        ray_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        *attenuation = self.albedo;
        *scattered = Ray::new(
            ray_in.at(rec.t),
            refleract(ray_in.direction, rec.normal, 0f64, self.fuzz),
        );
        true
    }
}
