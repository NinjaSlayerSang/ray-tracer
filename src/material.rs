mod lambertian;

use crate::{color::Color, hittable::HitRecord, ray::Ray};

pub trait Material {
    fn scatter(
        &self,
        _ray_in: &Ray,
        _rec: &HitRecord,
        _attenuation: &mut Color,
        _scattered: &mut Ray,
    ) -> bool {
        false
    }
}

impl Material for () {}
