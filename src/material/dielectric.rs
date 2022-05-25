use crate::{color::Color, hittable::HitRecord, ray::Ray, utils::refleract};

use super::Material;

#[derive(Clone, Copy)]
pub struct Dielectric {
    ir: f64,
}

impl Default for Dielectric {
    fn default() -> Self {
        Self { ir: 1f64 }
    }
}

impl Dielectric {
    pub fn new(ir: f64) -> Self {
        Self { ir: ir.abs() }
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
        *attenuation = Color::white();
        *scattered = Ray {
            origin: ray_in.at(rec.t),
            direction: refleract(ray_in.direction, rec.normal, self.ir, -1f64),
            time: ray_in.time,
        };
        true
    }
}
