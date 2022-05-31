use rand::{thread_rng, Rng};
use std::{cmp::Ordering, sync::Arc};

use crate::{aabb::Aabb, ray::Ray};

use super::{HitRecord, Hittable};

#[derive(Clone)]
pub struct BVHNode {
    left: Option<Arc<dyn Hittable + Send + Sync>>,
    right: Option<Arc<dyn Hittable + Send + Sync>>,
    bounding_box: Aabb,
}

impl BVHNode {
    pub fn from_list(list: &[Arc<dyn Hittable + Send + Sync>], time_range: (f64, f64)) -> Self {
        Self::build(&list.iter().collect::<Vec<_>>(), time_range)
    }

    fn build(src_objects: &[&Arc<dyn Hittable + Send + Sync>], time_range: (f64, f64)) -> Self {
        let mut objects = Vec::from(src_objects);
        let len = objects.len();

        let axis = thread_rng().gen_range(0u8..=2u8);
        objects.sort_by(|l, r| {
            use Ordering::{Equal, Greater, Less};
            match (
                l.try_bounding_box(time_range),
                r.try_bounding_box(time_range),
            ) {
                (None, None) => Equal,
                (None, Some(_)) => Less,
                (Some(_), None) => Greater,
                (Some(lb), Some(rb)) => Aabb::compare(&lb, &rb, axis).unwrap(),
            }
        });

        let mut left: Option<Arc<dyn Hittable + Send + Sync>> = None;
        let mut right: Option<Arc<dyn Hittable + Send + Sync>> = None;

        match len {
            0 => {}
            1 => {
                left = Some(objects[0].clone());
            }
            2 => {
                left = Some(objects[0].clone());
                right = Some(objects[1].clone());
            }
            _ => {
                let mid = (len + 1) / 2;
                left = Some(Arc::new(Self::build(&objects[0..mid], time_range)));
                right = Some(Arc::new(Self::build(&objects[mid..len], time_range)));
            }
        }

        let bounding_box = match (
            left.as_ref()
                .and_then(|opt| opt.try_bounding_box(time_range)),
            right
                .as_ref()
                .and_then(|opt| opt.try_bounding_box(time_range)),
        ) {
            (None, None) => Aabb::default(),
            (None, Some(rb)) => rb,
            (Some(lb), None) => lb,
            (Some(lb), Some(rb)) => Aabb::surround(lb, rb),
        };

        Self {
            left,
            right,
            bounding_box,
        }
    }
}

impl Hittable for BVHNode {
    fn hit(&self, ray: &Ray, t_range: (f64, f64), rec: &mut HitRecord) -> bool {
        self.bounding_box.hit(ray, t_range)
            && (self
                .left
                .as_ref()
                .map_or(false, |opt| opt.hit(ray, t_range, rec))
                | self
                    .right
                    .as_ref()
                    .map_or(false, |opt| opt.hit(ray, t_range, rec)))
    }

    fn bounding_box(&self, _: (f64, f64), output_box: &mut Aabb) -> bool {
        *output_box = self.bounding_box;
        true
    }
}
