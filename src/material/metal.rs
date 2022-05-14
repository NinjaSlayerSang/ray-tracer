use crate::{
    color::Color,
    ray::Ray,
    utils::{random_unit_vec3, reflect_unit_vec3},
    vec3::Vec3,
};

use super::Material;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
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
        ray_in: &crate::ray::Ray,
        rec: &crate::hittable::HitRecord,
        attenuation: &mut Color,
        scattered: &mut crate::ray::Ray,
    ) -> bool {
        let reflected = reflect_unit_vec3(ray_in.direction, rec.normal);
        *attenuation = self.albedo;
        *scattered = Ray::new(rec.point, reflected + self.fuzz * random_unit_vec3());
        Vec3::dot(reflected, rec.normal).is_sign_positive()
    }
}
