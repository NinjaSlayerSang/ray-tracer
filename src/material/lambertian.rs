use std::sync::Arc;

use crate::{
    color::Color,
    hittable::HitRecord,
    ray::Ray,
    texture::{SolidColor, Texture},
    vec3::Vec3,
};

use super::Material;

#[derive(Clone)]
pub struct Lambertian {
    albedo: Arc<dyn Texture + Send + Sync>,
}

impl Lambertian {
    pub fn new(color: Color) -> Self {
        Self {
            albedo: Arc::new(SolidColor::new(color)),
        }
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

            *attenuation = self.albedo.value(rec.ctx);
            *scattered = Ray {
                origin: ray_in.at(rec.t),
                direction: scatter_direction,
                time: ray_in.time,
            };
            true
        } else {
            *attenuation = Color::default();
            false
        }
    }
}
