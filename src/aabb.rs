use std::{cmp::Ordering, mem::swap};

use crate::{point3::Point3, ray::Ray};

#[derive(Clone, Copy, Default)]
pub struct Aabb {
    a: Point3,
    b: Point3,
}

impl Aabb {
    pub fn new(a: Point3, b: Point3) -> Self {
        Self { a, b }
    }

    pub fn surround(self, other: Self) -> Self {
        Self {
            a: Point3::new(
                self.a.x().min(other.a.x()),
                self.a.y().min(other.a.y()),
                self.a.z().min(other.a.z()),
            ),
            b: Point3::new(
                self.b.x().max(other.b.x()),
                self.b.y().max(other.b.y()),
                self.b.z().max(other.b.z()),
            ),
        }
    }

    pub fn hit(&self, ray: &Ray, t_range: (f64, f64)) -> bool {
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
                if t_max < t_min {
                    return false;
                }
            }
        }
        true
    }

    pub fn compare(&self, other: &Self, axis: u8) -> Option<Ordering> {
        match axis {
            0 => self.a.x().partial_cmp(&other.a.x()),
            1 => self.a.y().partial_cmp(&other.a.y()),
            2 => self.a.z().partial_cmp(&other.a.z()),
            _ => None,
        }
    }
}
