use crate::{point3::Point3, ray::Ray, vec3::Vec3};

#[derive(Clone, Debug)]
pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new_regular(viewport_width: f64, viewport_height: f64, focal_length: f64) -> Self {
        let origin = Point3::default();
        let horizontal = Vec3::from((viewport_width, 0f64, 0f64));
        let vertical = Vec3::from((0f64, viewport_height, 0f64));
        let lower_left_corner =
            origin - horizontal / 2f64 - vertical / 2f64 - Vec3::from((0f64, 0f64, focal_length));

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
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin,
        )
    }
}
