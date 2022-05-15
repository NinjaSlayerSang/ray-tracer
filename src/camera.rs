use crate::{point3::Point3, ray::Ray, vec3::Vec3};

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Camera {
    pub origin: Point3,
    pub lower_left_corner: Point3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
}

impl Camera {
    pub fn new(
        look_from: Point3,
        look_at: Point3,
        vup: Vec3,
        vfov: f64,
        aspect_ratio: f64,
    ) -> Self {
        let theta = vfov.to_radians();
        let h = (theta / 2f64).tan();
        let viewport_height = 2f64 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = Point3::vector(look_at, look_from).unit();
        let u = Vec3::cross(vup, w);
        let v = Vec3::cross(w, u);

        let origin = look_from;
        let horizontal = viewport_width * u;
        let vertical = viewport_height * v;
        let lower_left_corner = origin - horizontal / 2 - vertical / 2 - w;

        Self {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        Ray::new(
            self.origin,
            Point3::vector(
                self.origin,
                self.lower_left_corner + s * self.horizontal + t * self.vertical,
            ),
        )
    }
}
