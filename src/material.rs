mod context;
mod dielectric;
mod lambertian;
mod light_source;
mod metal;

use crate::{color::Color, hittable::HitRecord, ray::Ray};

pub use {
    context::Context, dielectric::Dielectric, lambertian::Lambertian, light_source::LightSource,
    metal::Metal,
};

#[derive(Clone, Copy, Default)]
pub struct Empty;

pub trait Material {
    fn scatter(
        &self,
        ray_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool;
}

impl Material for Empty {
    fn scatter(&self, _: &Ray, _: &HitRecord, _: &mut Color, _: &mut Ray) -> bool {
        false
    }
}
