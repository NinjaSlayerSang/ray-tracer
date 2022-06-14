mod context;
mod dielectric;
mod diffuse_light;
mod lambertian;
mod metal;

use crate::{color::Color, hittable::HitRecord, ray::Ray};

pub use {
    context::Context, dielectric::Dielectric, diffuse_light::DiffuseLight, lambertian::Lambertian,
    metal::Metal,
};

#[derive(Clone, Copy, Default)]
pub struct Empty;

pub trait Material {
    #![allow(unused_variables)]

    fn scatter(
        &self,
        ray_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        false
    }

    fn emitted(&self, ctx: Context) -> Color {
        Color::default()
    }
}

impl Material for Empty {}
