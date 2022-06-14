use std::sync::Arc;

use crate::{color::Color, hittable::HitRecord, ray::Ray, texture::Texture, vec3::Vec3};

use super::Material;

#[derive(Clone)]
pub struct Lambertian {
    texture: Arc<dyn Texture + Send + Sync>,
}

impl Lambertian {
    pub fn new(texture: Arc<dyn Texture + Send + Sync>) -> Self {
        Self { texture }
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
        outward && {
            let mut scatter_direction = rec.normal + Vec3::random_unit();

            if scatter_direction == Vec3::default() {
                scatter_direction = rec.normal
            }

            *attenuation = self.texture.value(rec.ctx);
            *scattered = Ray {
                origin: ray_in.at(rec.t),
                direction: scatter_direction,
                time: ray_in.time,
            };
            true
        }
    }
}
