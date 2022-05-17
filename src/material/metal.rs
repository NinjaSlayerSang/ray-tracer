use crate::{
    color::{primary_color::WHITE, Color},
    hittable::HitRecord,
    ray::Ray,
    utils::refleract,
    vec3::Vec3,
};

use super::Material;

#[derive(Clone, Copy)]
pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Default for Metal {
    fn default() -> Self {
        Self {
            albedo: WHITE,
            fuzz: 0f64,
        }
    }
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
        let outward = Vec3::dot(ray_in.direction, rec.normal) <= 0f64;
        if outward {
            *attenuation = self.albedo;
            *scattered = Ray::new(
                ray_in.at(rec.t),
                refleract(ray_in.direction, rec.normal, -1f64, self.fuzz),
            );
            true
        } else {
            *attenuation = Color::default();
            false
        }
    }
}
