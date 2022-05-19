use crate::{color::Color, hittable::HitRecord, ray::Ray, vec3::Vec3};

use super::Material;

#[derive(Clone, Copy)]
pub struct Lambertian {
    albedo: Color,
}

impl Default for Lambertian {
    fn default() -> Self {
        Self {
            albedo: Color::white(),
        }
    }
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
        if outward {
            let mut scatter_direction = rec.normal + Vec3::random_unit();

            if scatter_direction == Vec3::default() {
                scatter_direction = rec.normal
            }

            *attenuation = self.albedo;
            *scattered = Ray::new(ray_in.at(rec.t), scatter_direction);
            true
        } else {
            *attenuation = Color::default();
            false
        }
    }
}
