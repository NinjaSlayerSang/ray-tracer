use crate::{
    color::{primary_color::WHITE, Color},
    hittable::HitRecord,
    ray::Ray,
    utils::{reflect_unit_normal, refract_unit_normal},
};

use super::Material;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Dielectric {
    pub ir: f64,
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

        let refraction_ratio = if rec.front_face {
            self.ir.recip()
        } else {
            self.ir
        };

        if let Some(refracted) = refract_unit_normal(ray_in.direction, rec.normal, refraction_ratio)
        {
            *scattered = Ray::new(rec.point, refracted);
        } else {
            let reflected = reflect_unit_normal(ray_in.direction, rec.normal);
            *scattered = Ray::new(rec.point, reflected);
        }

        true
    }
}
