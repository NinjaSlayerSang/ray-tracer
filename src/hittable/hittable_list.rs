use std::sync::Arc;

use crate::{aabb::AABB, ray::Ray};

use super::{HitRecord, Hittable};

#[derive(Clone, Default)]
pub struct HittableList {
    objects: Vec<Arc<dyn Hittable + Send + Sync>>,
}

impl HittableList {
    pub fn add(&mut self, object: Arc<dyn Hittable + Send + Sync>) {
        self.objects.push(object)
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_range: (f64, f64), rec: &mut HitRecord) -> bool {
        let mut hit_anything = false;
        let mut closest_so_far = t_range.1;

        for object in &self.objects {
            if object.hit(ray, (t_range.0, closest_so_far), rec) {
                hit_anything = true;
                closest_so_far = rec.t;
            }
        }

        hit_anything
    }

    fn bounding_box(&self, time_range: (f64, f64), output_box: &mut AABB) -> bool {
        if self.objects.is_empty() {
            false
        } else {
            let mut temp_box = AABB::default();
            let mut first_box = true;

            for object in &self.objects {
                if !object.bounding_box(time_range, &mut temp_box) {
                    return false;
                }
                *output_box = if first_box {
                    temp_box
                } else {
                    output_box.surround(temp_box)
                };
                first_box = false;
            }

            true
        }
    }
}
