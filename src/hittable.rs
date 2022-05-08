#![allow(unused)]

use crate::point3::Point3;
use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct HitRecord {
    p: Point3,
    n: Vec3,
    t: f64,
}

pub trait Hittable {
    fn hit(ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}
