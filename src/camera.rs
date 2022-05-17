use crate::{point3::Point3, ray::Ray, utils::random_vec3_in_unit_circle, vec3::Vec3};

#[derive(Clone, Copy, PartialEq)]
pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f64,
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
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = self.lens_radius * random_vec3_in_unit_circle();
        let offset = self.u * rd.x() + self.v * rd.y();
        let origin = self.origin + offset;
        Ray::new(
            origin,
            Point3::vector(
                origin,
                self.lower_left_corner + s * self.horizontal + t * self.vertical,
            ),
        )
    }
}
