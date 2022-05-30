use std::sync::Arc;

use crate::{
    aabb::Aabb,
    material::Material,
    point3::Point3,
    ray::Ray,
    utils::{solve_quadratic_equation, QuadraticEquationRealRoot},
    vec3::Vec3,
};

use super::{HitRecord, Hittable};

#[derive(Clone)]
pub struct Sphere {
    center: Point3,
    radius: f64,
    material: Arc<dyn Material + Send + Sync>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, material: Arc<dyn Material + Send + Sync>) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_range: (f64, f64), rec: &mut HitRecord) -> bool {
        use QuadraticEquationRealRoot::{Double, Single};

        let (t_min, t_max) = t_range;

        let center = self.center;
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

    fn bounding_box(&self, _: (f64, f64), output_box: &mut Aabb) -> bool {
        let radius = self.radius;
        let radius_vector = Vec3::new(radius, radius, radius);
        *output_box = Aabb::new(self.center - radius_vector, self.center + radius_vector);
        true
    }
}
