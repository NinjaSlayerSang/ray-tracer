use crate::{
    point3::Point3,
    ray::Ray,
    utils::{solve_quadratic_equation, QuadraticEquationRealRoot},
};

use super::{HitRecord, Hittable};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Self {
        Self {
            center,
            radius: radius.abs(),
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let &Self { center, radius } = self;
        let &Ray { origin, direction } = ray;
        let oc = origin - center;

        let a = direction * direction;
        let hb = direction * oc;
        let c = oc * oc - radius * radius;

        let solve = solve_quadratic_equation(a, hb, c);

        let mut update = |t: f64| -> bool {
            rec.t = t;
            rec.point = ray.at(rec.t);
            rec.set_face_normal(ray, (rec.point - center) / radius);
            true
        };

        match solve {
            QuadraticEquationRealRoot::Double(t1, t2) => {
                if t_min < t1 && t1 < t_max {
                    update(t1)
                } else {
                    if t_min < t2 && t2 < t_max {
                        update(t2)
                    } else {
                        false
                    }
                }
            }
            QuadraticEquationRealRoot::Single(t) => {
                if t_min < t && t < t_max {
                    update(t)
                } else {
                    false
                }
            }
            _ => false,
        }
    }
}
