use rand::{thread_rng, Rng};

use crate::{point3::Point3, ray::Ray, utils::random_xy_in_circle, vec3::Vec3};

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f64,
    time_range: (f64, f64),
}

impl Camera {
    pub fn new(
        look_from: Point3,
        look_at: Point3,
        vup: Vec3,
        vfov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
        time_range: (f64, f64),
    ) -> Self {
        let theta = vfov.to_radians();
        let h = (theta / 2f64).tan();
        let viewport_height = 2f64 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = Point3::vector(look_at, look_from).unit();
        let u = Vec3::cross(vup, w).unit();
        let v = Vec3::cross(w, u);

        let origin = look_from;
        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;
        let lower_left_corner = origin - horizontal / 2 - vertical / 2 - focus_dist * w;

        let lens_radius = aperture / 2f64;

        Self {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            u,
            v,
            w,
            lens_radius,
            time_range,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let (x, y) = random_xy_in_circle(self.lens_radius);
        let offset = x * self.u + y * self.v;
        let origin = self.origin + offset;
        Ray {
            origin,
            direction: Point3::vector(
                origin,
                self.lower_left_corner + s * self.horizontal + t * self.vertical,
            ),
            time: thread_rng().gen_range(self.time_range.0..=self.time_range.1),
        }
    }
}
