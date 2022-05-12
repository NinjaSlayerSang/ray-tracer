use std::ops::RangeInclusive;
use std::rc::Rc;

use crate::ray::Ray;

use super::{HitRecord, Hittable};

#[derive(Default)]
pub struct HittableList {
    objects: Vec<Rc<dyn Hittable>>,
}

impl HittableList {
    pub fn add(&mut self, object: impl Hittable + 'static) -> Rc<impl Hittable> {
        let ptr = Rc::new(object);
        let shared = ptr.clone();
        self.objects.push(ptr);
        shared
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_range: RangeInclusive<f64>, rec: &mut HitRecord) -> bool {
        let mut hit_anything = false;
        let t_range_start = *t_range.start();
        let mut closest_so_far = *t_range.end();

        for object in &self.objects {
            if object.hit(ray, t_range_start..=closest_so_far, rec) {
                hit_anything = true;
                closest_so_far = rec.t;
            }
        }

        hit_anything
    }
}
