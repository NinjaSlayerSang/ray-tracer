mod hit_record;
mod hittable_list;
mod moving_sphere;
mod sphere;

use crate::ray::Ray;

pub use {
    hit_record::HitRecord, hittable_list::HittableList, moving_sphere::MovingSphere, sphere::Sphere,
};

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_range: (f64, f64), rec: &mut HitRecord) -> bool;
}
