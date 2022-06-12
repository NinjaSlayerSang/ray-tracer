use std::{
    f64::consts::{PI, TAU},
    sync::Arc,
};

use crate::{
    aabb::Aabb,
    material::{Context, Material},
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
    axis: (Vec3, Vec3, Vec3),
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, material: Arc<dyn Material + Send + Sync>) -> Self {
        Self {
            center,
            radius,
            material,
            axis: (Vec3::new(1, 0, 0), Vec3::new(0, 1, 0), Vec3::new(0, 0, 1)),
        }
    }

    pub fn set_axis(mut self, axis: (Vec3, Vec3)) -> Self {
        let (main, deputy) = axis;

        let y = main.unit();
        let z = Vec3::cross(deputy, y).unit();
        let x = Vec3::cross(y, z);

        self.axis = (x, y, z);
        self
    }

    fn get_sphere_uvp(&self, n: Vec3, p: Point3) -> Context {
        let (x, y, z) = self.axis;

        let phi = (-Vec3::dot(n, z)).atan2(Vec3::dot(n, x)) + PI;
        let theta = (-Vec3::dot(n, y)).acos();

        Context::UVP {
            u: phi / TAU,
            v: theta / PI,
            p,
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
            let point = ray.at(rec.t);
            rec.t = t;
            rec.normal = Point3::vector(center, point) / radius; // unit vector
            rec.material = self.material.clone();
            rec.ctx = self.get_sphere_uvp(rec.normal, point);
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
