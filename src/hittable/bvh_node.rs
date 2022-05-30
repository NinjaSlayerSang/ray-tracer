use std::sync::Arc;

use crate::{aabb::AABB, ray::Ray};

use super::{HitRecord, Hittable};

#[derive(Clone)]
pub struct BVHNode {
    left: Option<Arc<BVHNode>>,
    right: Option<Arc<BVHNode>>,
    bounding: AABB,
}

impl Hittable for BVHNode {
    fn hit(&self, ray: &Ray, t_range: (f64, f64), rec: &mut HitRecord) -> bool {
        if self.bounding.hit(ray, t_range) {
            (if let Some(l) = &self.left {
                l.hit(ray, t_range, rec)
            } else {
                false
            }) || (if let Some(r) = &self.right {
                r.hit(ray, t_range, rec)
            } else {
                false
            })
        } else {
            false
        }
    }

    fn bounding_box(&self, _: (f64, f64), output_box: &mut AABB) -> bool {
        *output_box = self.bounding;
        true
    }
}
