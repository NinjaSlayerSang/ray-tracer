use std::sync::Arc;

use crate::{
    material::Material,
    point3::Point3,
    ray::Ray,
    utils::{solve_quadratic_equation, QuadraticEquationRealRoot},
    vec3::Vec3,
};

use super::{HitRecord, Hittable};

#[derive(Clone)]
pub struct MovingSphere {
    center_range: (Point3, Point3),
    time_range: (f64, f64),
    radius: f64,
    material: Arc<dyn Material + Send + Sync>,
}

impl MovingSphere {
    pub fn new(
        center_range: (Point3, Point3),
        time_range: (f64, f64),
        radius: f64,
        material: Arc<dyn Material + Send + Sync>,
    ) -> Self {
        Self {
            center_range,
            time_range,
            radius,
            material,
        }
    }

    pub fn center(&self, time: f64) -> Point3 {
        let (center0, center1) = self.center_range;
        let (time0, time1) = self.time_range;
        center0 + (time - time0) / (time1 - time0) * Vec3::vector(center0, center1)
    }
}

impl Hittable for MovingSphere {
    fn hit(&self, ray: &Ray, t_range: (f64, f64), rec: &mut HitRecord) -> bool {
        use QuadraticEquationRealRoot::{Double, Single};

        let (t_min, t_max) = t_range;

        let center = self.center(ray.time);
        let radius = self.radius;
        let origin = ray.origin;
        let direction = ray.direction;
        let oc = origin - center;

        let a = Vec3::dot(direction, direction);
        let hb = Vec3::dot(direction, oc);
        let c = Vec3::dot(oc, oc) - radius * radius;

        let solve = solve_quadratic_equation(a, hb, c);

        let mut update = |t: f64| -> bool {
            rec.t = t;
            rec.normal = Point3::vector(center, ray.at(rec.t)) / radius; // unit vector
            rec.material = self.material.clone();
            true
        };

        match solve {
            Double(t1, t2) => {
                if t_min < t1 && t1 < t_max {
                    update(t1)
                } else if t_min < t2 && t2 < t_max {
                    update(t2)
                } else {
                    false
                }
            }
            Single(t) => {
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
