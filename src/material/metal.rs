use std::sync::Arc;

use crate::{
    color::Color, hittable::HitRecord, ray::Ray, texture::Texture, utils::refleract, vec3::Vec3,
};

use super::Material;

#[derive(Clone)]
pub struct Metal {
    texture: Arc<dyn Texture + Send + Sync>,
    fuzz: f64,
    double_sided: bool,
}

impl Metal {
    pub fn new(texture: Arc<dyn Texture + Send + Sync>, fuzz: f64) -> Self {
        Self {
            texture,
            fuzz: fuzz.clamp(0f64, 1f64),
            double_sided: false,
        }
    }

    pub fn set_double_sided(mut self, double_sided: bool) -> Self {
        self.double_sided = double_sided;
        self
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
        (self.double_sided || Vec3::dot(ray_in.direction, rec.normal) <= 0f64) && {
            *attenuation = self.texture.value(rec.ctx);
            *scattered = Ray {
                origin: ray_in.at(rec.t),
                direction: refleract(ray_in.direction, rec.normal, -1f64, self.fuzz),
                time: ray_in.time,
            };
            true
        }
    }
}
