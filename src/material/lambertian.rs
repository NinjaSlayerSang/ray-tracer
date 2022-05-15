use crate::{color::Color, hittable::HitRecord, ray::Ray, utils::random_unit_vec3, vec3::Vec3};

use super::Material;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        ray_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let outward = Vec3::dot(ray_in.direction, rec.normal) <= 0f64;
        let mut scatter_direction =
            if outward { rec.normal } else { -rec.normal } + random_unit_vec3();

        if scatter_direction == Vec3::default() {
            scatter_direction = rec.normal
        }

        *attenuation = self.albedo;
        *scattered = Ray::new(ray_in.at(rec.t), scatter_direction);
        true
    }
}
