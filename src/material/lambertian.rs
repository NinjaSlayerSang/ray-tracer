use crate::{color::Color, hittable::HitRecord, ray::Ray, utils::random_unit_vec3, vec3::Vec3};

use super::Material;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Lambertian {
    pub albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        _: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction = rec.normal + random_unit_vec3();

        if scatter_direction == Vec3::default() {
            scatter_direction = rec.normal
        }

        *attenuation = self.albedo;
        *scattered = Ray::new(rec.point, scatter_direction);
        true
    }
}
