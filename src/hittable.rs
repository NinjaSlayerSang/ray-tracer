#![allow(unused)]

mod hit_record;
mod sphere;

use std::ops::RangeInclusive;

use crate::ray::Ray;
pub use hit_record::HitRecord;
pub use sphere::Sphere;

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_interval: RangeInclusive<f64>, rec: &mut HitRecord) -> bool;
}
