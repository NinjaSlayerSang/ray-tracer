use std::sync::Arc;

use crate::{aabb::Aabb, ray::Ray};

use super::{HitRecord, Hittable};

#[derive(Clone, Default)]
pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable + Send + Sync>>,
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

    fn bounding_box(&self, time_range: (f64, f64), output_box: &mut Aabb) -> bool {
        let mut has_box = false;
        let mut temp_box = Aabb::default();

        for object in &self.objects {
            if object.bounding_box(time_range, &mut temp_box) {
                *output_box = if has_box {
                    output_box.surround(temp_box)
                } else {
                    has_box = true;
                    temp_box
                };
            }
        }

        has_box
    }
}
