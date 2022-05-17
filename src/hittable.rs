mod hit_record;
mod hittable_list;
mod sphere;

use crate::ray::Ray;

pub use {hit_record::HitRecord, hittable_list::HittableList, sphere::Sphere};

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_range: (f64, f64), rec: &mut HitRecord) -> bool;
}
