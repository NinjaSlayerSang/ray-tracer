use std::sync::Arc;

use crate::{color::Color, hittable::HitRecord, ray::Ray, texture::Texture, vec3::Vec3};

use super::Material;

#[derive(Clone)]
pub struct Lambertian {
    texture: Arc<dyn Texture + Send + Sync>,
    double_sided: bool,
}

impl Lambertian {
    pub fn new(texture: Arc<dyn Texture + Send + Sync>) -> Self {
        Self {
            texture,
            double_sided: false,
        }
    }

    pub fn set_double_sided(mut self, double_sided: bool) -> Self {
        self.double_sided = double_sided;
        self
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
        (self.double_sided || Vec3::dot(ray_in.direction, rec.normal) <= 0f64) && {
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
