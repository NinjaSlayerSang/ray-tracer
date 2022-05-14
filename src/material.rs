mod dielectric;
mod lambertian;
mod metal;

use crate::{color::Color, hittable::HitRecord, ray::Ray};

pub use {dielectric::Dielectric, lambertian::Lambertian, metal::Metal};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
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
