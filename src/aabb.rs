use std::mem::swap;

use crate::{point3::Point3, ray::Ray};

#[derive(Clone, Copy)]
pub struct AABB {
    a: Point3,
    b: Point3,
}

impl AABB {
    pub fn new(a: Point3, b: Point3) -> Self {
        Self { a, b }
    }

    pub fn surround(l: Self, r: Self) -> Self {
        Self {
            a: Point3::new(
                f64::min(l.a.x(), r.a.x()),
                f64::min(l.a.y(), r.a.y()),
                f64::min(l.a.z(), r.a.z()),
            ),
            b: Point3::new(
                f64::max(l.b.x(), r.b.x()),
                f64::max(l.b.y(), r.b.y()),
                f64::max(l.b.z(), r.b.z()),
            ),
        }
    }

    fn hit(&self, ray: &Ray, t_range: (f64, f64)) -> bool {
        let (mut t_min, mut t_max) = t_range;
        let origin: [f64; 3] = ray.origin.into();
        let direction: [f64; 3] = ray.direction.into();
        let a: [f64; 3] = self.a.into();
        let b: [f64; 3] = self.b.into();
        for i in 0..3 {
            if direction[i] == 0f64 {
                if origin[i] < a[i] || origin[i] > b[i] {
                    return false;
                }
            } else {
                let invd = direction[i].recip();
                let mut ta = (a[i] - origin[i]) * invd;
                let mut tb = (b[i] - origin[i]) * invd;
                if invd.is_sign_negative() {
                    swap(&mut ta, &mut tb);
                }
                t_min = t_min.max(ta);
                t_max = t_max.min(tb);
                if t_max <= t_min {
                    return false;
                }
            }
        }
        true
    }
}
