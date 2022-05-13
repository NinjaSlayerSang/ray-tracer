use crate::{point3::Point3, ray::Ray, vec3::Vec3};

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Camera {
    pub origin: Point3,
    pub lower_left_corner: Point3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
}

impl Camera {
    pub fn new_regular(viewport_width: f64, viewport_height: f64, focal_length: f64) -> Self {
        let origin = Point3::default();
        let horizontal = Vec3::new(viewport_width, 0f64, 0f64);
        let vertical = Vec3::new(0f64, viewport_height, 0f64);
        let lower_left_corner =
            origin - horizontal / 2f64 - vertical / 2f64 - Vec3::new(0f64, 0f64, focal_length);

        Self {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            Point3::vector(
                self.origin,
                self.lower_left_corner + u * self.horizontal + v * self.vertical,
            ),
        )
    }
}
