mod hit_record;
mod hittable_list;
mod sphere;

use crate::ray::Ray;

pub use hit_record::HitRecord;
pub use hittable_list::HittableList;
pub use sphere::Sphere;

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}
