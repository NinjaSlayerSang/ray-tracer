mod bvh_node;
mod hit_record;
mod hittable_list;
mod moving_sphere;
mod sphere;

use crate::{aabb::Aabb, ray::Ray};

pub use {
    bvh_node::BVHNode, hit_record::HitRecord, hittable_list::HittableList,
    moving_sphere::MovingSphere, sphere::Sphere,
};

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_range: (f64, f64), rec: &mut HitRecord) -> bool;

    fn bounding_box(&self, time_range: (f64, f64), output_box: &mut Aabb) -> bool;

    fn try_bounding_box(&self, time_range: (f64, f64)) -> Option<Aabb> {
        let mut output_box = Aabb::default();
        if self.bounding_box(time_range, &mut output_box) {
            Some(output_box)
        } else {
            None
        }
    }
}
